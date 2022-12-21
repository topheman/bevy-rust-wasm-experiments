/**
 * Inspired by my own rust implementation ball.rs: https://github.com/topheman/rust-wasm-experiments/blob/master/crate/src/ball.rs
 * itself inspired by an implementation I made ten years ago in JavaScript:
 * - https://github.com/topheman/Ball.js
 * - https://github.com/topheman/bombs/blob/master/src/js/vendor/Ball.js
 */
use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

use crate::resizable::Viewport;

pub struct BallPlugin;

#[derive(Component, Inspectable)]
pub struct Ball {
    pub velocity_x: f32,
    pub velocity_y: f32,
    radius: f32,
    mass: f32,
    gravity: f32,
    elasticity: f32,
    friction: f32,
    pub speed_with_keyboard: f32,
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
        }
    }
}

#[derive(SystemLabel)]
enum SystemLabel {
    MoveBalls,
    HandleBallBallCollisions,
    HandleBallWallCollisions,
}

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_ball_ball_collisions.label(SystemLabel::HandleBallBallCollisions))
            .add_system(handle_ball_wall_collisions.label(SystemLabel::HandleBallWallCollisions))
            .add_system(move_balls_one_step.label(SystemLabel::MoveBalls));
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

fn handle_ball_ball_collisions() {}

fn handle_ball_wall_collisions(
    mut balls_query: Query<(&mut Ball, &mut Transform)>,
    viewport_res: Res<Viewport>,
) {
    for (mut ball, mut transform) in balls_query.iter_mut() {
        if (transform.translation.y + ball.radius / 2.0) > viewport_res.max_y() {
            ball.velocity_y = -ball.velocity_y * ball.elasticity;
            transform.translation.y = viewport_res.max_y() - ball.radius / 2.0;
        }
        if (transform.translation.y - ball.radius / 2.0) < viewport_res.min_y() {
            ball.velocity_y = -ball.velocity_y * ball.elasticity;
            transform.translation.y = viewport_res.min_y() + ball.radius / 2.0;
        }
        if (transform.translation.x + ball.radius / 2.0) > viewport_res.max_x() {
            ball.velocity_x = -ball.velocity_x * ball.elasticity;
            transform.translation.x = viewport_res.max_x() - ball.radius / 2.0;
        }
        if (transform.translation.x - ball.radius / 2.0) < viewport_res.min_x() {
            ball.velocity_x = -ball.velocity_x * ball.elasticity;
            transform.translation.x = viewport_res.min_x() + ball.radius / 2.0;
        }
    }
}
