use bevy::{
    // diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    // render::render_resource::Texture,
    window::PresentMode,
};

mod debug;
mod player;
mod resizable;
mod texture;

#[cfg(target_arch = "wasm32")]
mod wasm;

use debug::DebugPlugin;
use player::PlayerPlugin;
use resizable::ResizablePlugin;
use texture::TexturePlugin;

#[cfg(target_arch = "wasm32")]
use wasm::WasmPlugin;

pub const CLEAR_COLOR: Color = Color::rgb(1.0, 0.0, 0.0);

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(CLEAR_COLOR))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: 600.0,
                height: 600.0,
                title: "Bevy Rust Experiments".to_string(),
                resizable: true,
                cursor_visible: true,
                present_mode: PresentMode::AutoVsync,
                ..default()
            },
            ..default()
        }))
        .add_startup_system(setup)
        .add_plugin(ResizablePlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(TexturePlugin)
        .add_plugin(PlayerPlugin);

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(WasmPlugin);
    app.run()
}
