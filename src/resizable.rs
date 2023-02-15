use bevy::{prelude::*, window::WindowResized};

pub struct ResizablePlugin;

#[derive(Resource)]
pub struct Viewport {
    width: f32,
    height: f32,
    min_x: f32,
    max_x: f32,
    min_y: f32,
    max_y: f32,
}
/* Fields above are kept private for encapsulation, only exposing public data via getters */
impl Viewport {
    pub fn min_x(&self) -> f32 {
        return self.min_x;
    }
    pub fn max_x(&self) -> f32 {
        return self.max_x;
    }
    pub fn min_y(&self) -> f32 {
        return self.min_y;
    }
    pub fn max_y(&self) -> f32 {
        return self.max_y;
    }
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
        // println!(
        //     "{}",
        //     format!(
        //         "{:.1} x {:.1} | {} {}",
        //         e.width, e.height, viewport.width, viewport.height
        //     )
        // );
    }
}
