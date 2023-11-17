use godot::prelude::*;
use godot::engine::{Node3D, INode3D, CollisionObject3D};

use super::*;
use crate::player::{Player, PlayerState};

#[derive(GodotClass)]
#[class(base = Node3D)]
pub struct PlaySign {
    #[export]
    sign: Option<Gd<Sign>>,
    #[export]
    body: Option<Gd<CollisionObject3D>>,
    #[export]
    player: Option<Gd<Player>>,
    #[export]
    sign_list: Option<Gd<SignList>>,

    #[base]
    base: Base<Node3D>
}

#[godot_api]
impl INode3D for PlaySign {
    fn init(base: Base<Node3D>) -> Self { 
        Self {
            sign: None,
            body: None,
            player: None,
            sign_list: None,
            base 
        }
    }
}

#[godot_api]
impl PlaySign {
    #[func]
    fn on_shot(&mut self, body: Gd<CollisionObject3D>) {
        if let Some(sign_body) = &self.body {
            if body == *sign_body {
                self.set_is_on(false);
                let Some(ref sign_list) = self.sign_list else {return;};
                let Some(ref mut exit) = sign_list.bind().exit_sign(&self.base.clone().upcast()) else {return;};
                exit.bind_mut().set_is_on(false);
                self.start_duel();
            }
        }
    }

    fn start_duel(&mut self) {
        if let Some(ref mut player) = &mut self.player {
            player.bind_mut().player_state = PlayerState::PreDuel;
        }
    }

    #[func]
    pub fn set_is_on(&mut self, value: bool) {
        let Some(ref mut sign) = self.sign else {return;};
        sign.bind_mut().is_on = value;
    }
}
