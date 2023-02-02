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

use crate::enemies::Enemy;
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
    pub kind: BallKind,
}
impl Ball {
    // public constructor, only expose public fields
    pub fn new(velocity_x: f32, velocity_y: f32, radius: f32, kind: BallKind) -> Ball {
        Ball {
            velocity_x,
            velocity_y,
            radius,
            kind,
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
            kind: BallKind::Enemy,
        }
    }
}

pub enum CollisionEvent {
    BallWall,
    EnemyEnemy,
    PlayerEnemy,
}

#[derive(PartialEq, Inspectable)]
pub enum BallKind {
    Enemy,
    Player,
}

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(handle_ball_ball_collisions.run_in_state(GameState::Playing))
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
    pub mass: f32,
    pub elasticity: f32,
}

fn handle_ball_ball_collisions(
    mut enemies_query: Query<(&mut Ball, &mut Transform)>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let mut iter = enemies_query.iter_combinations_mut();
    while let Some([(mut ball_left, transform_left), (mut ball_right, transform_right)]) =
        iter.fetch_next()
    {
        let ball_info_left = BallInfo {
            velocity_x: ball_left.velocity_x,
            velocity_y: ball_left.velocity_y,
            radius: ball_left.radius,
            x: transform_left.translation.x,
            y: transform_left.translation.y,
            mass: ball_left.mass,
            elasticity: ball_left.elasticity,
        };
        let ball_info_right = BallInfo {
            velocity_x: ball_right.velocity_x,
            velocity_y: ball_right.velocity_y,
            radius: ball_right.radius,
            x: transform_right.translation.x,
            y: transform_right.translation.y,
            mass: ball_right.mass,
            elasticity: ball_right.elasticity,
        };
        if check_ball_ball_collision(&ball_info_left, &ball_info_right) {
            println!("collision {} {}", ball_left.radius, ball_right.radius);
            if let Some((
                (new_ball_left_velocity_x, new_ball_left_velocity_y),
                (new_ball_right_velocity_x, new_ball_right_velocity_y),
            )) = resolve_ball_ball_collision(&ball_info_left, &ball_info_right)
            {
                ball_left.velocity_x = new_ball_left_velocity_x;
                ball_left.velocity_y = new_ball_left_velocity_y;
                ball_right.velocity_x = new_ball_right_velocity_x;
                ball_right.velocity_y = new_ball_right_velocity_y;
                if ball_left.kind == BallKind::Player || ball_right.kind == BallKind::Player {
                    collision_events.send(CollisionEvent::PlayerEnemy);
                } else {
                    collision_events.send(CollisionEvent::EnemyEnemy);
                }
            }
        }
    }
}

fn handle_ball_wall_collisions(
    mut balls_query: Query<(&mut Ball, &mut Transform)>,
    viewport_res: Res<Viewport>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    for (mut ball, mut transform) in balls_query.iter_mut() {
        if (transform.translation.y + ball.radius) > viewport_res.max_y() {
            ball.velocity_y = -ball.velocity_y * ball.elasticity;
            transform.translation.y = viewport_res.max_y() - ball.radius;
            collision_events.send(CollisionEvent::BallWall);
        }
        if (transform.translation.y - ball.radius) < viewport_res.min_y() {
            ball.velocity_y = -ball.velocity_y * ball.elasticity;
            transform.translation.y = viewport_res.min_y() + ball.radius;
            collision_events.send(CollisionEvent::BallWall);
        }
        if (transform.translation.x + ball.radius) > viewport_res.max_x() {
            ball.velocity_x = -ball.velocity_x * ball.elasticity;
            transform.translation.x = viewport_res.max_x() - ball.radius;
            collision_events.send(CollisionEvent::BallWall);
        }
        if (transform.translation.x - ball.radius) < viewport_res.min_x() {
            ball.velocity_x = -ball.velocity_x * ball.elasticity;
            transform.translation.x = viewport_res.min_x() + ball.radius;
            collision_events.send(CollisionEvent::BallWall);
        }
    }
}

fn check_ball_ball_collision(ball_left: &BallInfo, ball_right: &BallInfo) -> bool {
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

fn get_vector_2d(ball_left: &BallInfo, ball_right: &BallInfo) -> Vector2D {
    return Vector2D::new(ball_left.x - ball_right.x, ball_left.y - ball_right.y);
}

fn resolve_ball_ball_collision(
    ball_left: &BallInfo,
    ball_right: &BallInfo,
) -> Option<((f32, f32), (f32, f32))> {
    const RESTITUTION: f32 = 0.85;

    //get the mtd
    let delta = get_vector_2d(&ball_left, &ball_right);
    let d = delta.get_length();
    // minimum translation distance to push balls apart after intersecting
    let mtd = delta.scale(((ball_left.radius + ball_right.radius) - d) / d);

    // resolve intersection --
    // inverse mass quantities
    let im1 = 1.0 / ball_left.mass;
    let im2 = 1.0 / ball_right.mass;

    // impact speed
    let vector_velocity = Vector2D::new(
        ball_left.velocity_x - ball_right.velocity_x,
        ball_left.velocity_y - ball_right.velocity_y,
    );
    let normalized_mtd = mtd.normalize();
    let vn = vector_velocity.dot(&normalized_mtd);

    // sphere intersecting but moving away from each other already
    if vn > 0.0 {
        return None;
    }

    // collision impulse
    let i = (-(1.0 + RESTITUTION) * vn) / (im1 + im2);
    let impulse = normalized_mtd.scale(i);

    // change in momentum
    let ims1 = impulse.scale(im1);
    let ims2 = impulse.scale(im2);

    let new_ball_left_velocity_x = (ball_left.velocity_x + ims1.x) * ball_left.elasticity;
    let new_ball_left_velocity_y = (ball_left.velocity_y + ims1.y) * ball_left.elasticity;
    let new_ball_right_velocity_x = (ball_right.velocity_x - ims2.x) * ball_left.elasticity;
    let new_ball_right_velocity_y = (ball_right.velocity_y - ims2.y) * ball_left.elasticity;

    return Some((
        (new_ball_left_velocity_x, new_ball_left_velocity_y),
        (new_ball_right_velocity_x, new_ball_right_velocity_y),
    ));
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

/**
 * Inpired by my own port from js to rust: https://github.com/topheman/rust-wasm-experiments/blob/master/crate/src/vector2D.rs
 */

pub struct Vector2D {
    pub x: f32,
    pub y: f32,
}

impl Vector2D {
    pub fn new(x: f32, y: f32) -> Vector2D {
        Vector2D { x, y }
    }
    pub fn get_length(&self) -> f32 {
        f32::sqrt(self.x * self.x + self.y * self.y)
        // Currently using Math.sqrt from the browser - consider using rust implementation ?
        // (self.x * self.x + self.y * self.y).sqrt()
    }
    pub fn dot(&self, vector: &Vector2D) -> f32 {
        self.x * vector.x + self.y * vector.y
    }
    pub fn normalize(&self) -> Vector2D {
        Vector2D {
            x: self.x / self.get_length(),
            y: self.y / self.get_length(),
        }
    }
    pub fn scale(&self, scale: f32) -> Vector2D {
        Vector2D {
            x: self.x * scale,
            y: self.y * scale,
        }
    }
}
