use godot::prelude::*;
mod player;
mod dummy;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}