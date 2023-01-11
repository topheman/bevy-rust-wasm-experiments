use bevy::prelude::*;
use iyes_loopless::prelude::{AppLooplessStateExt, CurrentState, IntoConditionalSystem};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

use crate::ball::Ball;
use crate::state::{start_game, GameState};
use crate::texture::{spawn_assets_sprite, BallTexture};

pub const PLAYER_SCALE: f32 = 1.0;
pub const BALL_DEFAULT_RADIUS: f32 = 100.0;

pub struct PlayerPlugin;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    fn get_orientation_x() -> f32;
    fn get_orientation_y() -> f32;
}

#[cfg(not(target_arch = "wasm32"))]
fn get_orientation_x() -> f32 {
    return 0.0;
}

#[cfg(not(target_arch = "wasm32"))]
fn get_orientation_y() -> f32 {
    return 0.0;
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::PrepareGame, spawn_player)
            .add_system(handle_player_input_keyboard.run_in_state(GameState::Playing));
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
    if keyboard.pressed(KeyCode::Space) {
        ball.velocity_x = ball.velocity_x * 0.98;
        ball.velocity_y = ball.velocity_y * 0.98;
    }

    // mobile with accelerometer
    // todo fix ball/wall collision - velocity should be incremented ?
    let orientation_x = get_orientation_x();
    let orientation_y = get_orientation_y();
    if orientation_x != 0.0 && orientation_y != 0.0 {
        ball.velocity_x += orientation_x * ball.speed_with_accelerometer * time.delta_seconds();
        ball.velocity_y += orientation_y * ball.speed_with_accelerometer * time.delta_seconds();
    }
}

fn spawn_player(
    mut commands: Commands,
    ball_texture: Res<BallTexture>,
    gamestate: Res<CurrentState<GameState>>,
) {
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

    // once the player is ready, lets start the game - if we need other resources, start game after spawning them
    start_game(commands, gamestate);
}
