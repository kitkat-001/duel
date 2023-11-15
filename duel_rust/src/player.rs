use godot::prelude::*;
use godot::prelude::real_consts::FRAC_PI_2;
use godot::engine::{INode3D, Node3D, InputEvent, InputEventAction, InputEventMouseMotion, RayCast3D};
use godot::engine::utilities::clampf;

#[derive(GodotClass)]
#[class(base=Node3D)]
struct Player {
    camera: Option<Gd<Camera3D>>,
    raycast: Option<Gd<RayCast3D>>,
    delta: real,

    #[base]
    base: Base<Node3D>
}

#[godot_api]
impl INode3D for Player {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            camera: None,
            raycast: None,
            delta: 0f32, 
            base
        } 
    }

    fn process(&mut self, delta: f64) {
        if self.camera.is_none() || self.raycast.is_none() { self.set_nodes(); }
        self.delta = delta as f32;
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        self.set_camera_rotation(event)

    }
}

impl Player {
    fn set_nodes(&mut self) {
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
        if self.base.get_child_count() > 1 {
            let raycast_node = self.base.get_child(1);
            self.raycast = {
                if let Some(node) = raycast_node {
                    node.try_cast::<RayCast3D>()
                } else {
                    None
                }
            };
        }
    }

    fn set_camera_rotation(&mut self, event: Gd<InputEvent>) {
        if let Some(camera) = &mut self.camera {
            let mouse_event = event.try_cast::<InputEventMouseMotion>();
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

    fn shoot(&self, event: Gd<InputEvent>) {
        let mouse_event = event.try_cast::<InputEventAction>();
        if let Some(mouse_event) = mouse_event {
            if mouse_event.action() == StringName::from("shoot") {
                
            }
        }
    }
}