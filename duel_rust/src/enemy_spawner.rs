use godot::prelude::*;
use godot::engine::{Node3D, INode3D};

use super::player::*;
use super::dummy::*;

#[derive(GodotClass)]
#[class(base = Node3D)]
pub struct EnemySpawner {
    #[export]
    dummy_path: GString,
    #[export]
    player: Option<Gd<Player>>,
    #[base]
    base: Base<Node3D>
}

#[godot_api]
impl INode3D for EnemySpawner {
    fn init(base: Base<Node3D>) -> Self {
        Self {
            dummy_path: GString::from(""),
            player: None,
            base
        }
    }
}

#[godot_api]
impl EnemySpawner {
    pub fn spawn(&mut self) {
        let node = load::<PackedScene>(self.dummy_path.clone());
        let node = node.instantiate();
        if let Some(node) = node {
            if let Some(node) = node.try_cast::<Dummy>() {
                if let Some(player) = &mut self.player {
                    player.connect(StringName::from("shot"), node.callable("on_shot"));
                    self.base.add_child(node.upcast::<Node>());
                }
            }
        }
    }
}