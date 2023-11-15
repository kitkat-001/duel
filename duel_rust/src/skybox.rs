use godot::prelude::*;
use godot::engine::{StaticBody3D, IStaticBody3D, CollisionObject3D};

#[allow(unused_imports)]
use super::gd_print;

#[derive(GodotClass)]
#[class(base = StaticBody3D)]
struct Skybox {
    #[base]
    base: Base<StaticBody3D>
}

#[godot_api]
impl IStaticBody3D for Skybox {
    fn init(base: Base<StaticBody3D>) -> Self { Self {base} }
}

#[godot_api]
impl Skybox {
    #[func]
    fn on_shot(&self,  body: Gd<CollisionObject3D>) {
        if let Some(body) = body.try_cast::<StaticBody3D>() {
            if body == *self.base {
                gd_print("you lose");
            }
        }
    }
}