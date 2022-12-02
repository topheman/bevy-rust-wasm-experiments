use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::texture::{spawn_assets_sprite, BallTexture};

pub const PLAYER_SCALE: f32 = 1.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(player_movement.label("movement"));
    }
}

#[derive(Component, Inspectable)]
pub struct Player {
    speed: f32,
}

fn player_movement(
    mut player_query: Query<(&Player, &mut Transform)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (player, mut transform) = player_query.single_mut();

    if keyboard.pressed(KeyCode::Up) {
        transform.translation.y += 1.0 * player.speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::Down) {
        transform.translation.y -= 1.0 * player.speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::Left) {
        transform.translation.x -= 1.0 * player.speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::Right) {
        transform.translation.x += 1.0 * player.speed * time.delta_seconds();
    }
}

fn spawn_player(mut commands: Commands, ball_texture: Res<BallTexture>) {
    let player = spawn_assets_sprite(
        &mut commands,
        &ball_texture,
        1,
        Color::rgb(0.4, 0.9, 0.9),
        Vec3::new(0.0, 0.0, 900.0),
        Vec3::splat(PLAYER_SCALE),
    );

    commands
        .entity(player)
        .insert(Name::new("Player"))
        .insert(Player { speed: 200.0 });
}
