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

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EnemyEvents>()
            .add_system(spawn_enemies.run_in_state(GameState::Playing))
            .add_system(spawn_enemy.run_in_state(GameState::Playing));
    }
}

#[derive(Component)]
pub struct Enemy;

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
    mut spawn_events: EventWriter<EnemyEvents>,
) {
    let mut count = 0; // didn't found a method like .length or .size
    for _ in query_enemies.into_iter() {
        count += 1;
    }
    if count < MIN_ENEMIES {
        spawn_events.send(EnemyEvents::Spawn);
    }
}

/**
 * Actual instanciating, answering to an event emitted in spawn_enemies
 */
fn spawn_enemy(
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

    if enemy_count < MAX_ENEMIES {
        for event in spawn_events.iter() {
            match event {
                EnemyEvents::Spawn => {
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
                        Enemy,
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
                _ => {}
            }
        }
    }
}
