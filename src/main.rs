use bevy::prelude::*;
use bevy::window::WindowResolution;

mod audio;
mod ball;
mod colors;
mod debug;
mod enemies;
mod player;
mod resizable;
mod state;
mod stop_loop;
mod texture;
mod ui;

use audio::AudioPlugin;
use ball::{BallPlugin, CollisionEvent};
use debug::DebugPlugin;
use enemies::EnemiesPlugin;
use player::PlayerPlugin;
use resizable::ResizablePlugin;
use state::StatePlugin;
use stop_loop::StopLoopPlugin;
use texture::TexturePlugin;
use ui::UiPlugin;

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera {
            clear_color: ClearColorConfig::Custom(colors::DEFAULT_COLOR),
            ..default()
        },
    ));
}

fn main() {
    let mut app = App::new();
    app.insert_resource(ClearColor(Color::srgba(144.0 / 255.0, 0.0, 0.0, 1.0)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(600, 600),
                title: "Bevy Rust Experiments".to_string(),
                resizable: true,
                present_mode: bevy::window::PresentMode::AutoVsync,
                fit_canvas_to_parent: true,
                canvas: Some("#bevy".to_string()),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_plugins(StatePlugin)
        .add_plugins(ResizablePlugin)
        .add_plugins(DebugPlugin)
        .add_plugins(TexturePlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemiesPlugin)
        .add_plugins(BallPlugin)
        .add_message::<CollisionEvent>()
        .add_plugins(UiPlugin)
        .add_plugins(AudioPlugin)
        .add_plugins(StopLoopPlugin);

    app.run();
}
