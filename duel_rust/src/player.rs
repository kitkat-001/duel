use godot::prelude::*;
use godot::prelude::real_consts::FRAC_PI_2;
use godot::engine::{INode3D, Node3D, InputEvent, InputEventMouseMotion};
use godot::engine::utilities::clampf;

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
            delta: 0f32, 
            camera: None,
            base
        } 
    }

    fn process(&mut self, delta: f64) {
        self.delta = delta as f32;
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if self.camera.is_none() { self.set_camera(); }
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
}