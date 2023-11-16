use godot::prelude::*;
use godot::engine::{Node3D, INode3D, Label3D};

#[allow(unused_imports)]
use super::*;

mod play_sign;

#[derive(GodotClass)]
#[class(base = Node3D)]
pub struct Sign {
    #[export]
    off_position: Vector3,
    #[export]
    on_position: Vector3,
    #[export]
    speed: f32,
    #[export]
    pub is_on: bool,
    #[export]
    label: Option<Gd<Label3D>>,
    #[export]
    text: GString,

    #[base]
    base: Base<Node3D>
}


#[godot_api]
impl INode3D for Sign {
    fn init (base: Base<Node3D>) -> Self {
        Self {
            off_position: Vector3::ZERO,
            on_position: Vector3::ZERO,
            speed: 0.,
            is_on: false,
            label: None,
            text: GString::from(""),
            base
        }
    }

    fn ready(&mut self) {
        self.change_text(self.text.clone());
    }

    fn process(&mut self, delta: f64) {
        self.move_towards(if self.is_on {self.on_position} else {self.off_position}, delta as f32);
    }
}

#[godot_api]
impl Sign {
    pub fn change_text(&mut self, text: GString) {
        if let Some(label) = &mut self.label {
            label.set_text(text);
        }
    }

    fn move_towards(&mut self, dest: Vector3, delta: f32) {
        let curr = self.base.position();
        let displacement = dest - curr;
        let dir = displacement.normalized();
        let speed = min(self.speed * delta, displacement.length());
        self.base.set_position(curr + speed * dir);
    }
}



#[derive(GodotClass)]
#[class(base = Resource)]
pub struct SignList {
    #[export]
    result_sign: GString,

    #[base]
    _base: Base<Resource>
}

#[godot_api]
impl IResource for SignList {
    fn init(base: Base<Resource>) -> Self {
        Self {
            result_sign: GString::from(""),
            _base:  base
        }
    }
}

#[godot_api]
impl SignList {}