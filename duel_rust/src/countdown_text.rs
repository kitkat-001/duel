use godot::prelude::*;
use godot::engine::{Label, ILabel};

use super::player::*;

#[derive(GodotClass)]
#[class(base = Label)]
struct CountdownText {
    #[export]
    player: Option<Gd<Player>>,

    #[base]
    base: Base<Label>
}

#[godot_api]
impl ILabel for CountdownText {
    fn init(base: Base<Label>) -> Self {
        Self {
            player: None,
            base
        }
    }

    fn process(&mut self, _delta: f64) {
        if let Some(player) = &self.player {
            let player = player.bind();
            if player.player_state == PlayerState::PreDuel {
                let count = player.get_hop_count();
                let count = count + 1;
                if count > 0 && count <= 10 {
                    self.base.set_text(GString::from(count.to_string()))
                }
            }
            else {
                self.base.set_text(GString::from(""));
            }
        } else { self.base.set_text(GString::from("")); }
    }
}

#[godot_api] impl CountdownText{}
