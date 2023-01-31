/**
 * Inspired by my own rust implementation ball.rs: https://github.com/topheman/rust-wasm-experiments/blob/master/crate/src/ball.rs
 * itself inspired by an implementation I made ten years ago in JavaScript:
 * - https://github.com/topheman/Ball.js
 * - https://github.com/topheman/bombs/blob/master/src/js/vendor/Ball.js
 */
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use iyes_loopless::prelude::IntoConditionalSystem;
use rand::Rng;

use crate::ennemies::Ennemy;
use crate::player::Player;
use crate::resizable::Viewport;
use crate::state::GameState;

pub struct BallPlugin;

#[derive(Component, Inspectable)]
pub struct Ball {
    pub velocity_x: f32,
    pub velocity_y: f32,
    pub radius: f32,
    mass: f32,
    gravity: f32,
    elasticity: f32,
    pub friction: f32,
    pub speed_with_keyboard: f32,
    pub speed_with_accelerometer: f32,
}
impl Ball {
    // public constructor, only expose public fields
    pub fn new(velocity_x: f32, velocity_y: f32, radius: f32) -> Ball {
        Ball {
            velocity_x,
            velocity_y,
            radius,
            ..default()
        }
    }
}
impl Default for Ball {
    fn default() -> Self {
        Ball {
            velocity_x: 0.0,
            velocity_y: 0.0,
            radius: 100.0,
            mass: 1.0,
            gravity: 1.0,
            elasticity: 0.98,
            friction: 0.8,
            speed_with_keyboard: 200.0,
            speed_with_accelerometer: 400.0,
        }
    }
}

pub enum CollisionEvent {
    BallWall,
    EnnemyEnnemy,
    PlayerEnnemy,
}

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_ennemy_ennemy_collisions.run_in_state(GameState::Playing))
            // .add_system(handle_player_ennemy_collisions.run_in_state(GameState::Playing))
            .add_system(handle_ball_wall_collisions.run_in_state(GameState::Playing))
            .add_system(move_balls_one_step.run_in_state(GameState::Playing));
    }
}

fn move_balls_one_step(mut balls_query: Query<(&Ball, &mut Transform)>, time: Res<Time>) {
    for (ball, mut transform) in balls_query.iter_mut() {
        transform.translation.x =
            transform.translation.x + ball.gravity * ball.velocity_x * time.delta_seconds();
        transform.translation.y =
            transform.translation.y + ball.gravity * ball.velocity_y * time.delta_seconds();
    }
}

struct BallInfo {
    pub velocity_x: f32,
    pub velocity_y: f32,
    pub radius: f32,
    pub x: f32,
    pub y: f32,
}

fn handle_ennemy_ennemy_collisions(
    mut ennemies_query: Query<(&mut Ball, &mut Transform, With<Ennemy>)>,
    viewport_res: Res<Viewport>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let mut iter = ennemies_query.iter_combinations_mut();
    while let Some([(mut ball_left, transform_left, _), (mut ball_right, transform_right, _)]) =
        iter.fetch_next()
    {
        if check_ball_ball_collision(
            BallInfo {
                velocity_x: ball_left.velocity_x,
                velocity_y: ball_left.velocity_y,
                radius: ball_left.radius,
                x: transform_left.translation.x,
                y: transform_left.translation.y,
            },
            BallInfo {
                velocity_x: ball_right.velocity_x,
                velocity_y: ball_right.velocity_y,
                radius: ball_right.radius,
                x: transform_right.translation.x,
                y: transform_right.translation.y,
            },
        ) {
            println!(
                "implement collision {} {}",
                ball_left.radius, ball_right.radius
            )
        }
    }
    // for (mut ball_left, transform_left, _) in ennemies_query.iter_mut() {
    //     for (mut ball_right, transform_right, _) in ennemies_query.iter_mut() {
    //         if check_ball_ball_collision(
    //             BallInfo {
    //                 velocity_x: ball_left.velocity_x,
    //                 velocity_y: ball_left.velocity_y,
    //                 radius: ball_left.radius,
    //                 x: transform_left.translation.x,
    //                 y: transform_left.translation.y,
    //             },
    //             BallInfo {
    //                 velocity_x: ball_right.velocity_x,
    //                 velocity_y: ball_right.velocity_y,
    //                 radius: ball_right.radius,
    //                 x: transform_right.translation.x,
    //                 y: transform_right.translation.y,
    //             },
    //         ) {
    //             println!("collision")
    //         }
    //     }
    // }
}

// fn handle_player_ennemy_collisions(
//     mut ennemies_query: Query<(&mut Ball, &mut Transform, With<Ennemy>)>,
//     mut player_query: Query<(&mut Ball, &mut Transform, With<Player>)>,
//     viewport_res: Res<Viewport>,
//     mut collision_events: EventWriter<CollisionEvent>,
// ) {
// }

fn handle_ball_wall_collisions(
    mut balls_query: Query<(&mut Ball, &mut Transform)>,
    viewport_res: Res<Viewport>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    for (mut ball, mut transform) in balls_query.iter_mut() {
        if (transform.translation.y + ball.radius / 2.0) > viewport_res.max_y() {
            ball.velocity_y = -ball.velocity_y * ball.elasticity;
            transform.translation.y = viewport_res.max_y() - ball.radius / 2.0;
            collision_events.send(CollisionEvent::BallWall);
        }
        if (transform.translation.y - ball.radius / 2.0) < viewport_res.min_y() {
            ball.velocity_y = -ball.velocity_y * ball.elasticity;
            transform.translation.y = viewport_res.min_y() + ball.radius / 2.0;
            collision_events.send(CollisionEvent::BallWall);
        }
        if (transform.translation.x + ball.radius / 2.0) > viewport_res.max_x() {
            ball.velocity_x = -ball.velocity_x * ball.elasticity;
            transform.translation.x = viewport_res.max_x() - ball.radius / 2.0;
            collision_events.send(CollisionEvent::BallWall);
        }
        if (transform.translation.x - ball.radius / 2.0) < viewport_res.min_x() {
            ball.velocity_x = -ball.velocity_x * ball.elasticity;
            transform.translation.x = viewport_res.min_x() + ball.radius / 2.0;
            collision_events.send(CollisionEvent::BallWall);
        }
    }
}

fn check_ball_ball_collision(ball_left: BallInfo, ball_right: BallInfo) -> bool {
    let xd = ball_left.x - ball_right.x;
    let yd = ball_left.y - ball_right.y;

    let sum_radius = ball_left.radius + ball_right.radius;
    let sqr_radius = sum_radius * sum_radius;

    let dist_sqr = xd * xd + yd * yd;

    if dist_sqr <= sqr_radius {
        return true;
    }
    return false;
}

fn get_safe_random_position(
    window_size: f32,
    safe_zone_min: f32,
    safe_zone_max: f32,
) -> (f32, bool) {
    let mut rng = rand::thread_rng();
    let random_position_from_center: f32 =
        rng.gen_range(((-window_size) / 2.0)..((window_size) / 2.0));
    if random_position_from_center > safe_zone_min && random_position_from_center < safe_zone_max {
        return (random_position_from_center, false);
    } else {
        return (random_position_from_center, true);
    };
}

fn get_safe_random_positions(
    window_width: f32,
    window_height: f32,
    safe_zone_min_x: f32,
    safe_zone_max_x: f32,
    safe_zone_min_y: f32,
    safe_zone_max_y: f32,
) -> (f32, f32) {
    let result = loop {
        println!("get_safe_random_positions");
        let (x, unsafe_x) =
            get_safe_random_position(window_width, safe_zone_min_x, safe_zone_max_x);
        let (y, unsafe_y) =
            get_safe_random_position(window_height, safe_zone_min_y, safe_zone_max_y);
        if unsafe_x || unsafe_y || (!unsafe_x && !unsafe_y) {
            break (x, y);
        }
    };
    return result;
}

fn get_random_velocity(position: f32, player_position: f32, max_velocity: f32) -> f32 {
    let mut rng = rand::thread_rng();
    let mut velocity: f32 = if position > player_position {
        max_velocity
    } else {
        -max_velocity
    };
    velocity *= rng.gen::<f32>();
    return velocity;
}

pub fn get_random_position_and_speed(
    window_width: f32,
    window_height: f32,
    safe_zone_min_x: f32,
    safe_zone_max_x: f32,
    safe_zone_min_y: f32,
    safe_zone_max_y: f32,
    player_x: f32,
    player_y: f32,
    max_velocity: f32,
) -> (Vec3, f32, f32) {
    let (x, y) = get_safe_random_positions(
        window_width,
        window_height,
        safe_zone_min_x,
        safe_zone_max_x,
        safe_zone_min_y,
        safe_zone_max_y,
    );
    let velocity_x = get_random_velocity(x, player_x, max_velocity);
    let velocity_y = get_random_velocity(y, player_y, max_velocity);
    let translation = Vec3::new(x / 2.0, y / 2.0, 900.0);
    println!("velocity {:?} {:?}", velocity_x, velocity_y);
    return (translation, velocity_x, velocity_y);
}
