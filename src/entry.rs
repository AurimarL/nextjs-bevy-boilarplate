use bevy::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console;

fn hello_world() {
    console::log_1(&"hello world".into()); // Use web_sys::console::log_1 to send messages to the browser's console
}

#[wasm_bindgen]
pub fn run_agame_app() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, hello_world)
        .run();
}
