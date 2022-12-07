use bevy::prelude::*;
use wasm_bindgen::prelude::*;

pub struct WasmPlugin;

impl Plugin for WasmPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(resizer);
    }
}

#[wasm_bindgen]
extern "C" {
    fn resize_canvas(width: f32, height: f32);
}

fn resizer() {
    let window = web_sys::window().expect("no global `window` exists");
    let width: f32 = window.inner_width().unwrap().as_f64().unwrap() as f32;
    let height: f32 = window.inner_height().unwrap().as_f64().unwrap() as f32;
    resize_canvas(width, height);
}
