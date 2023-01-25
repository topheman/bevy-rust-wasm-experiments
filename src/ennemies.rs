use bevy::prelude::*;

use crate::ball::{get_random_position_and_speed, Ball};
use crate::player::Player;
use crate::state::GameState;
use crate::texture::{spawn_assets_sprite, BallTexture};
use iyes_loopless::prelude::{AppLooplessStateExt, IntoConditionalSystem};

pub const ENNEMY_SCALE: f32 = 0.7;
pub const BALL_DEFAULT_RADIUS: f32 = 100.0;
pub const MIN_ENNEMIES: usize = 2;

pub struct EnnemiesPlugin;

impl Plugin for EnnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EnnemyEvents>()
            .add_system(spawn_ennemies.run_in_state(GameState::Playing))
            .add_system(spawn_ennemy.run_in_state(GameState::Playing));
    }
}

#[derive(Component)]
pub struct Ennemy {
    life: f32,
}

enum EnnemyEvents {
    Spawn,
    Kill(Entity),
}

impl Ennemy {
    fn bounce_wall(&mut self) {
        self.life -= 2.0;
    }
    fn bounce_player(&mut self) {
        self.life -= 4.0;
    }
    fn is_dying(&self) -> bool {
        return self.life < 6.0;
    }
}

/**
 * Orchestrates the spwaning of ennemies according to their number
 */
fn spawn_ennemies(
    query_ennemies: Query<(Entity, With<Ennemy>)>,
    mut spawn_events: EventWriter<EnnemyEvents>,
) {
    let mut count = 0; // didn't found a method like .length or .size
    for _ in query_ennemies.into_iter() {
        count += 1;
    }
    if count < 2 {
        spawn_events.send(EnnemyEvents::Spawn);
    }
}

/**
 * Actual instanciating, answering to an event emitted in spawn_ennemies
 */
fn spawn_ennemy(
    mut commands: Commands,
    ball_texture: Res<BallTexture>,
    windows: Res<Windows>,
    query_player: Query<(&Ball, &Transform, With<Player>)>,
    mut spawn_events: EventReader<EnnemyEvents>,
) {
    for event in spawn_events.iter() {
        match event {
            EnnemyEvents::Spawn => {
                println!("spawn_ennemy");
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
                let ennemy_ball_component = (
                    Ball::new(velocity_x, velocity_y, BALL_DEFAULT_RADIUS * ENNEMY_SCALE),
                    Ennemy { life: 20.0 },
                );
                let player_entity = spawn_assets_sprite(
                    &mut commands,
                    &ball_texture,
                    1,
                    Color::rgb(0.4, 0.5, 0.5),
                    random_position,
                    Vec3::splat(ENNEMY_SCALE),
                );

                commands
                    .entity(player_entity)
                    .insert(ennemy_ball_component)
                    .insert(Name::new("Ennemy"));
            }
            _ => {}
        }
    }
}
