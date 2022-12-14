use bevy::{prelude::*, window::WindowResized};

pub struct ResizablePlugin;

/**
 * Todo: find a way not to break encapsulation.
 * The viewport resource is exposed to the whole app and can be mutated by any system 💣
 */
#[derive(Resource)]
pub struct Viewport {
    pub width: f32,
    pub height: f32,
    pub min_x: f32,
    pub max_x: f32,
    pub min_y: f32,
    pub max_y: f32,
}

impl Default for Viewport {
    fn default() -> Self {
        Viewport {
            width: 0.0,
            height: 0.0,
            min_x: 0.0,
            max_x: 0.0,
            min_y: 0.0,
            max_y: 0.0,
        }
    }
}

impl Plugin for ResizablePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Viewport { ..default() });
        app.add_system(on_resize_system);
    }
}

fn on_resize_system(
    mut resize_reader: EventReader<WindowResized>,
    mut viewport_res: ResMut<Viewport>,
) {
    for e in resize_reader.iter() {
        let viewport = viewport_res.as_mut();
        viewport.width = e.width;
        viewport.height = e.height;
        viewport.min_x = -e.width / 2.0;
        viewport.max_x = e.width / 2.0;
        viewport.min_y = -e.height / 2.0;
        viewport.max_y = e.height / 2.0;
        println!(
            "{}",
            format!(
                "{:.1} x {:.1} | {} {}",
                e.width, e.height, viewport.width, viewport.height
            )
        );
    }
}
