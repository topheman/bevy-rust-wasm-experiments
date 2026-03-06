use bevy::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

use crate::ball::{Ball, BallKind};
use crate::enemies::EnemyEvents;
use crate::state::{start_game, GameState};
use crate::texture::{spawn_assets_sprite, BallTexture};

pub const PLAYER_SCALE: f32 = 1.0;
pub const BALL_DEFAULT_RADIUS: f32 = 50.0;

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
        app.add_systems(OnEnter(GameState::PrepareGame), spawn_player)
            .add_systems(Update, handle_player_input.run_if(in_state(GameState::Playing)));
    }
}

#[derive(Component)]
pub struct Player;

fn handle_player_input(
    mut player_query: Query<&mut Ball, With<Player>>,
    mouse: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    touches: Res<Touches>,
    mut spawn_events: MessageWriter<EnemyEvents>,
    time: Res<Time>,
) {
    let mut ball = player_query.single_mut().unwrap();

    if keyboard.pressed(KeyCode::ArrowUp) {
        ball.velocity_y += ball.speed_with_keyboard * time.delta_secs();
    }
    if keyboard.pressed(KeyCode::ArrowDown) {
        ball.velocity_y -= ball.speed_with_keyboard * time.delta_secs();
    }
    if keyboard.pressed(KeyCode::ArrowLeft) {
        ball.velocity_x -= ball.speed_with_keyboard * time.delta_secs();
    }
    if keyboard.pressed(KeyCode::ArrowRight) {
        ball.velocity_x += ball.speed_with_keyboard * time.delta_secs();
    }
    if keyboard.just_pressed(KeyCode::Space) || mouse.just_pressed(MouseButton::Left) || touches.any_just_pressed() {
        spawn_events.write(EnemyEvents::Spawn);
    }

    // mobile with accelerometer
    let orientation_x = get_orientation_x();
    let orientation_y = get_orientation_y();
    if orientation_x != 0.0 && orientation_y != 0.0 {
        ball.velocity_x += orientation_x * ball.speed_with_accelerometer * time.delta_secs();
        ball.velocity_y += orientation_y * ball.speed_with_accelerometer * time.delta_secs();
    }
}

fn spawn_player(
    mut commands: Commands,
    ball_texture: Res<BallTexture>,
    gamestate: Res<State<GameState>>,
) {
    let player_ball_component = (
        Ball::new(
            30.0,
            40.0,
            BALL_DEFAULT_RADIUS * PLAYER_SCALE,
            BallKind::Player,
        ),
        Player,
    );
    let player_entity = spawn_assets_sprite(
        &mut commands,
        &ball_texture,
        0,
        Color::srgb(0.4, 0.9, 0.9),
        Vec3::new(0.0, 0.0, 900.0),
        Vec3::splat(PLAYER_SCALE),
    );

    commands
        .entity(player_entity)
        .insert(player_ball_component)
        .insert(Name::new("Player"));

    start_game(commands, gamestate);
}
