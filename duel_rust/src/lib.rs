use godot::prelude::*;
mod player;
mod dummy;
mod skybox;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

#[allow(dead_code)]
fn gd_print(value: &str) {
    print(&[value.to_variant()])
}