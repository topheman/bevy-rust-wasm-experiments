use bevy::prelude::*;

use crate::ball::Ball;
use crate::texture::{spawn_assets_sprite, BallTexture};

pub const PLAYER_SCALE: f32 = 1.0;
pub const BALL_DEFAULT_RADIUS: f32 = 100.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_system(handle_player_input_keyboard);
    }
}

fn handle_player_input_keyboard(
    mut player_query: Query<&mut Ball>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let mut ball = player_query.single_mut();

    if keyboard.pressed(KeyCode::Up) {
        ball.velocity_y += ball.speed_with_keyboard * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::Down) {
        ball.velocity_y -= ball.speed_with_keyboard * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::Left) {
        ball.velocity_x -= ball.speed_with_keyboard * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::Right) {
        ball.velocity_x += ball.speed_with_keyboard * time.delta_seconds();
    }
}

fn spawn_player(mut commands: Commands, ball_texture: Res<BallTexture>) {
    let player_ball_component = Ball::new(30.0, 40.0, BALL_DEFAULT_RADIUS * PLAYER_SCALE);
    let player_entity = spawn_assets_sprite(
        &mut commands,
        &ball_texture,
        1,
        Color::rgb(0.4, 0.9, 0.9),
        Vec3::new(0.0, 0.0, 900.0),
        Vec3::splat(PLAYER_SCALE),
    );

    commands
        .entity(player_entity)
        .insert(player_ball_component)
        .insert(Name::new("Player"));
}
