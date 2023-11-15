use godot::prelude::*;
use godot::prelude::real_consts::FRAC_PI_2;
use godot::engine::{Node3D, INode3D, StaticBody3D, CollisionObject3D};

#[allow(unused_imports)]
use super::gd_print;

#[derive(GodotClass)]
#[class(base=Node3D)]
struct Dummy {
    head: Option<Gd<StaticBody3D>>,
    body: Option<Gd<StaticBody3D>>,
    is_dead: bool,
    #[export]
    fall_speed: f32,

    #[base]
    base: Base<Node3D>
}

#[godot_api]
impl INode3D for Dummy {
    fn init(base: Base<Node3D>) -> Self { 
        Self {
            head: None,
            body: None,
            is_dead: false,
            fall_speed: 0.,
            base
        }
    }

    fn ready(&mut self) {
        self.get_colliders();
    }

    fn process(&mut self, delta: f64) {
        if self.is_dead 
        {
            if self.base.rotation().x < FRAC_PI_2 {
                self.base.rotate(Vector3::RIGHT, self.fall_speed * delta as f32);
            } else {
                self.base.queue_free();
            }
        }
    }
}

#[godot_api]
impl Dummy {
    #[func]
    fn on_shot(&mut self, body: Gd<CollisionObject3D>)  {
        let body = body.try_cast::<StaticBody3D>();
        if let Some(body) = body {
            if let Some(head) = &self.head {
                if body == *head {
                    self.is_dead = true;
                }
            }
            if let Some(dummy_body) = &self.body {
                if body == *dummy_body {
                    self.is_dead = true;
                }
            }
        }
    }
}

impl Dummy {
    fn get_colliders(&mut self) {
        if self.base.get_child_count() > 1 {
            let head_node = self.base.get_child(0);
            self.head = {
                if let Some(node) = head_node {
                    node.try_cast::<StaticBody3D>()
                } else {
                    None
                }
            };
            let body_node = self.base.get_child(1);
            self.body = {
                if let Some(node) = body_node {
                    node.try_cast::<StaticBody3D>()
                } else {
                    None
                }
            };
        }
    }
}