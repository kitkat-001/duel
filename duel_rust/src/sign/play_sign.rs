use godot::prelude::*;
use godot::engine::{Node3D, INode3D, CollisionObject3D};

use super::*;
use crate::player::{Player, PlayerState};

#[derive(GodotClass)]
#[class(base = Node3D)]
struct PlaySign {
    #[export]
    sign: Option<Gd<Sign>>,
    #[export]
    body: Option<Gd<CollisionObject3D>>,
    #[export]
    player: Option<Gd<Player>>,

    #[base]
    _base: Base<Node3D>
}

#[godot_api]
impl INode3D for PlaySign {
    fn init(base: Base<Node3D>) -> Self { 
        Self {
            sign: None,
            body: None,
            player: None,
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
                    self.start_duel();
                }
            }
        }
    }

    fn start_duel(&mut self) {
        if let Some(ref mut player) = &mut self.player {
            player.bind_mut().player_state = PlayerState::PreDuel;
        }
    }
}
