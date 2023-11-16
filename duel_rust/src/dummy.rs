use godot::prelude::*;
use godot::prelude::real_consts::FRAC_PI_2;
use godot::engine::{Node3D, INode3D, StaticBody3D, CollisionObject3D};

use super::sign::*;

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
    #[export]
    sign_list: Option<Gd<SignList>>,

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
            sign_list: None,
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
                    self.activate_sign(GString::from("headshot!!!"));
                }
            }
            if let Some(dummy_body) = &self.body {
                if body == *dummy_body {
                    self.is_dead = true;
                    self.activate_sign(GString::from("you win!"));
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

    fn activate_sign(&mut self, text: GString) {
        if let Some(sign_list) = &mut self.sign_list {
            if let Some(sign) = self.base.get_node(NodePath::from(sign_list.bind_mut().get_result_sign())) {
                if let Some(mut sign) = sign.try_cast::<Sign>() {
                    sign.bind_mut().is_on = true;
                    sign.bind_mut().change_text(text);
                }
            }
        }
    }
}