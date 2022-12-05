use bevy::{prelude::*, window::WindowResized};

pub struct ResizablePlugin;

impl Plugin for ResizablePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(on_resize_system);
    }
}

fn on_resize_system(mut resize_reader: EventReader<WindowResized>) {
    for e in resize_reader.iter() {
        println!("{}", format!("{:.1} x {:.1}", e.width, e.height));
    }
}
