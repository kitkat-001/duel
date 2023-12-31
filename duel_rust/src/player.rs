use godot::prelude::*;
use godot::prelude::real_consts::{PI, FRAC_PI_2};
use godot::engine::{CharacterBody3D, ICharacterBody3D, InputEvent, InputEventMouseMotion, PhysicsRayQueryParameters3D, CollisionObject3D, ProjectSettings, Timer};
use godot::engine::input::MouseMode;
use godot::engine::utilities::clampf;

#[allow(unused_imports)]
use super::*;
use super::enemy_spawner::*;
use super::sign::*;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum PlayerState {
    NotDueling,
    PreDuel,
    Duel(bool)
}

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
pub struct Player {
    camera: Option<Gd<Camera3D>>,
    pub player_state: PlayerState,
    delta: f32,
    pub hop_count: i32,

    #[export]
    camera_speed: f32,
    #[export]
    timer: Option<Gd<Timer>>,
    duel_timer: f64,
    #[export]
    enemy_spawner: Option<Gd<EnemySpawner>>,
    #[export]
    sign_list: Option<Gd<SignList>>,
    #[export]
    fall_speed: f32,
    pub is_dead: bool,
    rotation: f32,

    #[base]
    base: Base<CharacterBody3D>
}

#[godot_api]
impl ICharacterBody3D for Player {
    fn init(base: Base<CharacterBody3D>) -> Self {
        Self {
            camera: None,
            player_state: PlayerState::NotDueling,
            delta: 0., 
            hop_count: 10,
            camera_speed: 0.,
            timer: None,
            duel_timer: 0.,
            enemy_spawner: None,
            sign_list: None,
            fall_speed: 0.,
            is_dead: false,
            rotation: 0.,
            base
        } 
    }

    fn ready(&mut self) {
        Input::singleton().set_mouse_mode(MouseMode::MOUSE_MODE_HIDDEN);
        Input::singleton().set_mouse_mode(MouseMode::MOUSE_MODE_CAPTURED);
        self.set_camera();
    }

    fn process(&mut self, delta: f64) {
        if self.is_dead {
            self.die();
            return;
        } else {
            self.undie();
        }

        if let PlayerState::Duel(var) = self.player_state { 
            if var { self.duel_timer += delta; }
        } else {
            self.duel_timer = 0.;
        }
        self.delta = delta as f32;

        if self.player_state == PlayerState::PreDuel {
            let is_facing_backwards = self.face_backwards();
            if let Some(ref mut timer) = &mut self.timer {
                if is_facing_backwards && timer.is_stopped() {
                    timer.start();
                    self.hop(false);
                    if let Some(spawner) = &mut self.enemy_spawner {
                        spawner.bind_mut().spawn();
                    }
                }
            }
        }

        if self.player_state == PlayerState::Duel(false) && self.base.is_on_floor() && self.hop_count > 0 {
            self.hop(true);
            if self.hop_count == 0 {
                self.player_state = PlayerState::NotDueling;
            }
        }
        self.do_gravity();
        self.do_friction();
        self.base.move_and_slide();
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if self.player_state != PlayerState::PreDuel {
            self.set_camera_rotation(&event);
            if self.can_shoot() || self.player_state == PlayerState::Duel(false) {
                self.shoot(&event);
            }
        }
    }
}

#[godot_api]
impl Player {
    #[signal]
    pub fn shot(body: Gd<CollisionObject3D>);

    #[func]
    fn timeout(&mut self) {
        if self.hop_count > 0 {
            self.hop(self.player_state != PlayerState::PreDuel);
        } else {
            if let Some(ref mut timer) = &mut self.timer {
                timer.stop();
            }
            self.player_state = PlayerState::Duel(true);
        }
    }
}

impl Player {
    fn set_camera(&mut self) {
        if self.base.get_child_count() > 0 {
            let cam_node = self.base.get_child(0);
            self.camera = {
                if let Some(node) = cam_node {
                    node.try_cast::<Camera3D>()
                } else {
                    None
                }
            };
        }
    }

    fn set_camera_rotation(&mut self, event: &Gd<InputEvent>) {
        if let Some(camera) = &mut self.camera {
            let mouse_event = event.clone().try_cast::<InputEventMouseMotion>();
            if let Some(mouse_event) = mouse_event {
                let relative = mouse_event.relative() * self.delta / 4f32;
                let curr = camera.rotation();
                camera.set_rotation(Vector3{ 
                    x: clampf((curr.x - relative.y) as f64, -FRAC_PI_2 as f64, FRAC_PI_2 as f64) as f32,
                    y: curr.y - relative.x,
                    z: curr.z
                });
            }
        }
    }

    fn shoot(&mut self, event: &Gd<InputEvent>) -> Option<()> {
        let camera = self.camera.clone()?;
        if event.is_action_pressed(StringName::from("shoot")) {
            let Some(ref sign_list) = self.sign_list else {return None;};
            let Some(ref result_sign) = sign_list.bind().result_sign(&self.base.clone().upcast()) else {return None;};
            let Some(ref death_sign) = sign_list.bind().death_sign(&self.base.clone().upcast()) else {return None;};
            if self.player_state == PlayerState::Duel(false) {
                if self.hop_count == 0 && !result_sign.bind().in_motion && !death_sign.bind().in_motion {
                    self.hop_count = 10;
                    let Some(ref mut sign_list) = self.sign_list else { return None; };
                    let sign = self.base.get_node(NodePath::from(sign_list.bind().get_result_sign()))?;
                    sign.try_cast::<Sign>()?.bind_mut().is_on = false;
                    let sign_list = sign_list.bind_mut();
                    sign_list.play_sign(&self.base.clone().upcast())?.bind_mut().set_is_on(true);
                    sign_list.exit_sign(&self.base.clone().upcast())?.bind_mut().set_is_on(true);
                    self.is_dead = false;
                    if let Some(mut dummy) = self.base.get_node_or_null(NodePath::from("../EnemySpawner/Dummy")) {
                        dummy.queue_free();
                    }
                }
                return Some(())
            }
            self.use_ammo();
            let query = PhysicsRayQueryParameters3D::create(
                self.base.global_position() + 1.5 * Vector3::UP, 
                self.base.global_position() + 1.5 * Vector3::UP - 100. * camera.global_transform().basis.col_c()
            )?;
            let collider = self.base.world_3d()?
                .direct_space_state()?
                .intersect_ray(query)
                .get(StringName::from("collider"))?;
            self.base.call_deferred(StringName::from("emit_signal"), &[StringName::from("shot").to_variant(), collider.to_variant()]);
        }
        Some(())
    }

    fn can_shoot(&self) -> bool {
        [PlayerState::NotDueling, PlayerState::Duel(true)].contains(&self.player_state)
    }

    fn use_ammo(&mut self) {
        if let PlayerState::Duel(_) = self.player_state {
            self.player_state = PlayerState::Duel(false);
        }
    }

    fn face_backwards(&mut self) -> bool {
        if let Some(camera) = &mut self.camera {
            let curr = camera.rotation();
            let target = Vector3 {x: 0., y: PI, z: 0.};
            let displacement = target - curr;
            let dir = displacement.normalized();
            let speed = min(self.camera_speed, displacement.length());
            camera.set_rotation(curr + dir * speed);
            dir == Vector3::ZERO
        } else { true } // Escape if camera can't be found.
    }

    fn hop(&mut self, is_going_forwards: bool) {
        self.hop_count -= 1;
        self.base.set_velocity(Vector3 { x: 0., y: 2.45, z: 2. * if is_going_forwards {1.} else {-1.} });
    }

    fn do_gravity(&mut self) {
        let curr = self.base.velocity();
        let grav_vector = ProjectSettings::singleton().get_setting(GString::from("physics/3d/default_gravity_vector")).to::<Vector3>();
        let grav_magnitude = ProjectSettings::singleton().get_setting(GString::from("physics/3d/default_gravity")).to::<f32>();
        self.base.set_velocity(curr + grav_vector * grav_magnitude * self.delta);
    }

    fn do_friction(&mut self) {
        if self.base.is_on_floor() {
            let y = self.base.velocity().y;
            if y <= 0. {
                self.base.set_velocity(Vector3::UP * y);
            }
        }
    }

    pub fn get_hop_count(&self) -> i32 {
        self.hop_count
    }

    pub fn get_duel_timer(&self) -> f64 {
        self.duel_timer
    }

    fn die(&mut self) {
        self.use_ammo();
        if self.rotation > -FRAC_PI_2  {
            self.base.rotate(Vector3::RIGHT, -self.fall_speed * self.delta);
            self.rotation += -self.fall_speed * self.delta;
        }
        
        let Some(ref sign_list) = self.sign_list else {return;};
        let Some(ref mut death_sign) = sign_list.bind().death_sign(&self.base.clone().upcast()) else {return;};
        death_sign.bind_mut().is_on = true;
    }

    fn undie(&mut self) {
        if self.rotation < 0. {
            self.base.rotate(Vector3::RIGHT, self.fall_speed * self.delta);
            self.rotation += self.fall_speed * self.delta;
        }

        let Some(ref sign_list) = self.sign_list else {return;};
        let Some(ref mut death_sign) = sign_list.bind().death_sign(&self.base.clone().upcast()) else {return;};
        death_sign.bind_mut().is_on = false;
    }
}