use godot::prelude::*;
use godot::engine::{Node3D, INode3D, StaticBody3D};

#[derive(GodotClass)]
#[class(base=Node3D)]
struct Dummy {
    head: Option<Gd<StaticBody3D>>,
    body: Option<Gd<StaticBody3D>>,

    #[base]
    base: Base<Node3D>
}

#[godot_api]
impl INode3D for Dummy {
    fn init(base: Base<Node3D>) -> Self { 
        Self {
            head: None,
            body: None,
            base
        }
    }

    fn process(&mut self, _delta: f64) {
        if self.head.is_none() || self.body.is_none() { self.get_colliders(); }
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