use std::time::Duration;

use bevy::prelude::*;

use crate::ball::{get_random_position_and_speed, Ball, BallKind};
use crate::player::Player;
use crate::state::GameState;
use crate::texture::{spawn_assets_sprite, BallTexture};
use iyes_loopless::prelude::IntoConditionalSystem;

pub const ENEMY_SCALE: f32 = 0.7;
pub const BALL_DEFAULT_RADIUS: f32 = 50.0;
pub const MIN_ENEMIES: i32 = 2;
pub const MAX_ENEMIES: i32 = 10;
const LIFESPAN_SECS: u64 = 14;
const DYING_DURATION: f32 = 0.2;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EnemyEvents>()
            .add_system(spawn_enemies.run_in_state(GameState::Playing))
            .add_system(kill_enemies.run_in_state(GameState::Playing))
            .add_system(spawn_or_kill_enemy.run_in_state(GameState::Playing));
    }
}

#[derive(Component)]
pub struct Enemy {
    life_timer: Timer,
    dying_timer: Timer,
}

impl Default for Enemy {
    fn default() -> Self {
        Enemy {
            life_timer: Timer::new(Duration::from_secs(LIFESPAN_SECS), TimerMode::Once),
            dying_timer: Timer::new(Duration::from_secs_f32(DYING_DURATION), TimerMode::Once),
        }
    }
}

impl Enemy {
    pub fn new() -> Enemy {
        Enemy { ..default() }
    }
    pub fn is_dying(&self) -> bool {
        self.life_timer.finished()
    }
    pub fn is_dead(&self) -> bool {
        self.dying_timer.finished()
    }
    pub fn tick(&mut self, delta: Duration) {
        if !self.is_dying() {
            self.life_timer.tick(delta);
        } else {
            self.dying_timer.tick(delta);
        }
    }
}

pub enum EnemyEvents {
    Spawn,
    #[allow(unused)]
    Kill(Entity),
}

/**
 * Orchestrates the spwaning of enemies according to their number
 */
fn spawn_enemies(
    query_enemies: Query<(Entity, With<Enemy>)>,
    mut enemy_events: EventWriter<EnemyEvents>,
) {
    let mut count = 0; // didn't found a method like .length or .size
    for _ in query_enemies.into_iter() {
        count += 1;
    }
    if count < MIN_ENEMIES {
        enemy_events.send(EnemyEvents::Spawn);
    }
}

fn kill_enemies(
    mut query_enemies: Query<(Entity, &mut Enemy, &mut Transform, &mut Ball)>,
    mut enemy_events: EventWriter<EnemyEvents>,
    time: Res<Time>,
) {
    let mut count = 0; // didn't found a method like .length or .size
    for _ in query_enemies.iter() {
        count += 1;
    }

    if count > MIN_ENEMIES {
        for (_, mut enemy, _, _) in query_enemies.iter_mut() {
            enemy.tick(time.delta());
        }
        for (entity, enemy, mut transform, mut ball) in query_enemies.iter_mut() {
            if enemy.is_dead() {
                enemy_events.send(EnemyEvents::Kill(entity));
            } else if enemy.is_dying() {
                transform.scale *= 0.80;
                ball.radius *= 0.80;
                println!("scale {:?} radius {:?}", transform.scale, ball.radius);
            };
        }
    }
}

/**
 * Actual instanciating, answering to an event emitted in spawn_enemies
 */
fn spawn_or_kill_enemy(
    mut commands: Commands,
    ball_texture: Res<BallTexture>,
    windows: Res<Windows>,
    query_player: Query<(&Ball, &Transform, With<Player>)>,
    query_enemies: Query<(Entity, With<Enemy>)>,
    mut spawn_events: EventReader<EnemyEvents>,
) {
    let mut enemy_count = 0;
    for _ in query_enemies.into_iter() {
        enemy_count += 1;
    }

    for event in spawn_events.iter() {
        match event {
            EnemyEvents::Spawn => {
                if enemy_count < MAX_ENEMIES {
                    let player = query_player.single();
                    let player_x = player.1.translation.x;
                    let player_y = player.1.translation.y;
                    let player_radius = player.0.radius;
                    let window = windows.get_primary().unwrap();
                    let (random_position, velocity_x, velocity_y) = get_random_position_and_speed(
                        window.width(),
                        window.height(),
                        player_x - player_radius,
                        player_x + player_radius,
                        player_y - player_radius,
                        player_y + player_radius,
                        player_x,
                        player_y,
                        200.0,
                    );
                    let enemy_ball_component = (
                        Ball::new(
                            velocity_x,
                            velocity_y,
                            BALL_DEFAULT_RADIUS * ENEMY_SCALE,
                            BallKind::Enemy,
                        ),
                        Enemy::new(),
                    );
                    let player_entity = spawn_assets_sprite(
                        &mut commands,
                        &ball_texture,
                        1,
                        Color::rgb(0.4, 0.5, 0.5),
                        random_position,
                        Vec3::splat(ENEMY_SCALE),
                    );

                    commands
                        .entity(player_entity)
                        .insert(enemy_ball_component)
                        .insert(Name::new("Enemy"));
                }
            }
            EnemyEvents::Kill(entity) => {
                // todo make a noise
                /*
                 `commands.entity(*entity).despawn()` panics in bevy/dynamic mode
                  So we make sure we have an EntityCommands before we access it.
                */
                if let Some(mut entity_command) = commands.get_entity(*entity) {
                    entity_command.despawn();
                }
            }
        }
    }
}
