use bevy::prelude::*;
use bevy::window::WindowResized;
use wasm_bindgen::prelude::*;

pub struct WasmPlugin;

impl Plugin for WasmPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LastSize {
            width: 0.0,
            height: 0.0,
        });
        app.add_system(resizer);
    }
}

#[wasm_bindgen]
extern "C" {
    fn resize_canvas(width: f32, height: f32);
}

/**
 * Track the last size of the window in order to decide whether we need to resize the canvas on the web page or not
 * (resizer is system executed in loop)
 */
#[derive(Resource)]
struct LastSize {
    pub width: f32,
    pub height: f32,
}

/**
 * Inspired by https://github.com/horup/some-tank-game-rs/blob/main/src/wasm/mod.rs
 */
fn resizer(
    mut windows: ResMut<Windows>,
    mut last_size: ResMut<LastSize>,
    mut window_resized_events: EventWriter<WindowResized>,
) {
    let browser_window = web_sys::window().expect("no global `window` exists");
    let width: f32 = browser_window.inner_width().unwrap().as_f64().unwrap() as f32;
    let height: f32 = browser_window.inner_height().unwrap().as_f64().unwrap() as f32;

    if let Some(window) = windows.get_primary_mut() {
        if width != last_size.width || height != last_size.height {
            last_size.width = width;
            last_size.height = height;

            // physical_pixels = logical_pixels * scale_factor
            let physical_width = width * window.scale_factor() as f32;
            let physical_height = height * window.scale_factor() as f32;
            window.update_actual_size_from_backend(physical_width as u32, physical_height as u32);
            window_resized_events.send(WindowResized {
                id: window.id(),
                height,
                width,
            });
            resize_canvas(width, height);
        }
    }
}
