use bevy::prelude::*;

use crate::ball::{get_random_position_and_speed, Ball};
use crate::state::GameState;
use crate::texture::{spawn_assets_sprite, BallTexture};
use iyes_loopless::prelude::AppLooplessStateExt;

pub const ENNEMY_SCALE: f32 = 0.7;
pub const BALL_DEFAULT_RADIUS: f32 = 100.0;

pub struct EnnemiesPlugin;

impl Plugin for EnnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::PrepareGame, spawn_ennemy);
    }
}

#[derive(Component)]
pub struct Ennemy {
    life: f32,
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

fn spawn_ennemy(mut commands: Commands, ball_texture: Res<BallTexture>, windows: Res<Windows>) {
    println!("spawn_ennemy");
    let window = windows.get_primary().unwrap();
    let (random_position, velocity_x, velocity_y) =
        get_random_position_and_speed(window.width(), window.height(), 350.0, 100.0);
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
