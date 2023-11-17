use godot::prelude::*;
use godot::engine::{StaticBody3D, IStaticBody3D, CollisionObject3D};

use super::sign::*;
use super::player::*;

#[allow(unused_imports)]
use super::gd_print;

#[derive(GodotClass)]
#[class(base = StaticBody3D)]
struct Skybox {
    #[export]
    sign_list: Option<Gd<SignList>>,
    #[export]
    player: Option<Gd<Player>>,

    #[base]
    base: Base<StaticBody3D>
}

#[godot_api]
impl IStaticBody3D for Skybox {
    fn init(base: Base<StaticBody3D>) -> Self { 
        Self {
            sign_list: None,
            player: None,
            base
        } 
    }
}

#[godot_api]
impl Skybox {
    #[func]
    fn on_shot(&mut self,  body: Gd<CollisionObject3D>) {
        let Some(ref player) = self.player else {return;};
        if player.bind().player_state == PlayerState::NotDueling {
            return;
        }
        let time = player.bind().get_duel_timer();
        
        if let Some(body) = body.try_cast::<StaticBody3D>() {
            if body == *self.base {
                if let Some(sign_list) = &mut self.sign_list {
                    if let Some(sign) = self.base.get_node(NodePath::from(sign_list.bind_mut().get_result_sign())) {
                        if let Some(mut sign) = sign.try_cast::<Sign>() {
                            sign.bind_mut().is_on = true;
                            sign.bind_mut().change_text(GString::from(format!("you lose :(\n time: {:.3}", time)));
                        }
                    }
                }
            }
        }
    }
}