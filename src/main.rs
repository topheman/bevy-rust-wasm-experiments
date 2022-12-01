use bevy::{
    // diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    // render::render_resource::Texture,
    window::PresentMode,
};

mod debug;
mod player;
use debug::DebugPlugin;
use player::PlayerPlugin;

pub const CLEAR_COLOR: Color = Color::rgb(1.0, 0.0, 0.0);

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR_COLOR))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: 600.0,
                height: 600.0,
                title: "Bevy Rust Experiments".to_string(),
                resizable: false,
                cursor_visible: true,
                present_mode: PresentMode::AutoVsync,
                ..default()
            },
            ..default()
        }))
        .add_startup_system(setup)
        .add_plugin(DebugPlugin)
        .add_plugin(PlayerPlugin)
        .run()
}
