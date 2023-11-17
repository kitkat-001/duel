use godot::prelude::*;
use godot::engine::{Node3D, INode3D, CollisionObject3D};

#[allow(unused_imports)]
use super::*;

#[derive(GodotClass)]
#[class(base = Node3D)]
pub struct ExitSign {
    #[export]
    sign: Option<Gd<Sign>>,
    #[export]
    body: Option<Gd<CollisionObject3D>>,

    #[base]
    base: Base<Node3D>
}

#[godot_api]
impl INode3D for ExitSign {
    fn init(base: Base<Node3D>) -> Self { 
        Self {
            sign: None,
            body: None,
            base 
        }
    }
}

#[godot_api]
impl ExitSign {
    #[func]
    fn on_shot(&self, body: Gd<CollisionObject3D>) {
        if let Some(sign_body) = &self.body {
            if body == *sign_body {
                let Some(ref mut tree) = self.base.tree() else {return;};
                tree.quit();
            }
        }
    }

    pub fn set_is_on(&mut self, value: bool) {
        let Some(ref mut sign) = self.sign else {return;};
        sign.bind_mut().is_on = value;
    }
}
