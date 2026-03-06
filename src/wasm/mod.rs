use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowResized};
use wasm_bindgen::prelude::*;

/**
 * This is dead code.
 * Not needed anymore after bevy@0.9.1
 *
 * See: https://github.com/bevyengine/bevy/commit/fed93a0edce9d66586dc70c1207a2092694b9a7d
 */

pub struct WasmPlugin;

impl Plugin for WasmPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LastSize {
            width: 0.0,
            height: 0.0,
        });
        app.add_systems(Update, resizer);
    }
}

#[wasm_bindgen]
extern "C" {
    fn resize_canvas(width: f32, height: f32);
}

#[derive(Resource)]
struct LastSize {
    pub width: f32,
    pub height: f32,
}

fn resizer(
    mut window_query: Query<(Entity, &Window), With<PrimaryWindow>>,
    mut last_size: ResMut<LastSize>,
    mut window_resized_events: MessageWriter<WindowResized>,
) {
    let browser_window = web_sys::window().expect("no global `window` exists");
    let width: f32 = browser_window.inner_width().unwrap().as_f64().unwrap() as f32;
    let height: f32 = browser_window.inner_height().unwrap().as_f64().unwrap() as f32;

    if let Ok((entity, window)) = window_query.get_single_mut() {
        if width != last_size.width || height != last_size.height {
            last_size.width = width;
            last_size.height = height;

            window_resized_events.write(WindowResized {
                window: entity,
                height,
                width,
            });
            resize_canvas(width, height);
        }
    }
}
