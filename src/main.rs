use bevy::{
    // diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    // render::render_resource::Texture,
    window::PresentMode,
};

mod audio;
mod ball;
mod debug;
mod player;
mod resizable;
mod texture;

use audio::AudioPlugin;
use ball::{BallPlugin, CollisionEvent};
use debug::DebugPlugin;
use player::PlayerPlugin;
use resizable::ResizablePlugin;
use texture::TexturePlugin;

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
                fit_canvas_to_parent: true, // no more need to handle this myself with wasm binding: https://github.com/bevyengine/bevy/commit/fed93a0edce9d66586dc70c1207a2092694b9a7d
                canvas: Some("#bevy".to_string()),
                ..default()
            },
            ..default()
        }))
        .add_startup_system(setup)
        .add_plugin(ResizablePlugin)
        .add_plugin(DebugPlugin)
        .add_plugin(TexturePlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(BallPlugin)
        .add_event::<CollisionEvent>()
        .add_plugin(AudioPlugin);

    app.run()
}
