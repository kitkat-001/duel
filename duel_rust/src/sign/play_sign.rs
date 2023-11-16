use godot::prelude::*;
use godot::engine::{Node3D, INode3D, CollisionObject3D};

use super::*;

#[derive(GodotClass)]
#[class(base = Node3D)]
struct PlaySign {
    #[export]
    sign: Option<Gd<Sign>>,
    #[export]
    body: Option<Gd<CollisionObject3D>>,

    #[base]
    _base: Base<Node3D>
}

#[godot_api]
impl INode3D for PlaySign {
    fn init(base: Base<Node3D>) -> Self { 
        Self {
            sign: None,
            body: None,
            _base: base 
        }
    }
}

#[godot_api]
impl PlaySign {
    #[func]
    fn on_shot(&mut self, body: Gd<CollisionObject3D>) {
        if let Some(ref mut sign) =  &mut self.sign {
            if let Some(sign_body) = &self.body {
                if body == *sign_body {
                    sign.bind_mut().is_on = false;
                }
            }
        }
    }
}
