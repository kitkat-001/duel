use godot::prelude::*;
use godot::prelude::real_consts::FRAC_PI_2;
use godot::engine::{INode3D, Node3D, InputEvent, InputEventMouseMotion, PhysicsRayQueryParameters3D, CollisionObject3D};
use godot::engine::input::MouseMode;
use godot::engine::utilities::clampf;

#[allow(unused_imports)]
use super::gd_print;

#[derive(GodotClass)]
#[class(base=Node3D)]
struct Player {
    camera: Option<Gd<Camera3D>>,
    delta: real,

    #[base]
    base: Base<Node3D>
}

#[godot_api]
impl INode3D for Player {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            camera: None,
            delta: 0., 
            base
        } 
    }

    fn ready(&mut self) {
        Input::singleton().set_mouse_mode(MouseMode::MOUSE_MODE_HIDDEN);
        Input::singleton().set_mouse_mode(MouseMode::MOUSE_MODE_CAPTURED);
        self.set_camera();
    }

    fn process(&mut self, delta: f64) {
        self.delta = delta as f32;
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        self.set_camera_rotation(&event);
        self.shoot(&event);
    }
}

#[godot_api]
impl Player {
    #[signal]
    fn shot(body: Gd<CollisionObject3D>);
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
            let query = PhysicsRayQueryParameters3D::create(
                self.base.global_position() + 1.5 * Vector3::UP, 
                self.base.global_position() + 1.5 * Vector3::UP - 100. * camera.global_transform().basis.col_c()
            )?;
            let collider = self.base.world_3d()?
                .direct_space_state()?
                .intersect_ray(query)
                .get(StringName::from("collider"))?;
            self.base.emit_signal(StringName::from("shot"), &[collider.to_variant()]);
        }
        Some(())
    }
}