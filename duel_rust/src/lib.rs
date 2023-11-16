use godot::prelude::*;
mod player;
mod dummy;
mod skybox;
mod sign;

struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

#[allow(dead_code)]
fn gd_print(value: &str) {
    print(&[value.to_variant()])
}

fn min(a: f32, b: f32) -> f32 {if a < b {a} else {b}}