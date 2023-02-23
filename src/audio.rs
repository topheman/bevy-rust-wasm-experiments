use bevy::prelude::*;

use crate::ball::CollisionEvent;
use crate::enemies::EnemyEvents;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_sounds)
            .add_system(play_enemy_sounds)
            .add_system(play_collision_sounds);
    }
}

#[derive(Resource)]
struct BallWallSound(Handle<AudioSource>);

#[derive(Resource)]

struct EnemyDyingSound(Handle<AudioSource>);

fn load_sounds(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ball_wall_sound = asset_server.load("impactMetal_heavy_004.ogg");
    commands.insert_resource(BallWallSound(ball_wall_sound));
    let enemy_dying_sound = asset_server.load("impactPlate_medium_000.ogg");
    commands.insert_resource(EnemyDyingSound(enemy_dying_sound));
}

fn play_collision_sounds(
    mut collision_events: EventReader<CollisionEvent>,
    audio: Res<Audio>,
    ball_wall_sound: Res<BallWallSound>,
) {
    for event in collision_events.iter() {
        match event {
            CollisionEvent::BallWall(_) => audio.play(ball_wall_sound.0.clone()),
            _ => audio.play(ball_wall_sound.0.clone()),
        };
    }
}

fn play_enemy_sounds(
    mut enemy_events: EventReader<EnemyEvents>,
    audio: Res<Audio>,
    enemy_dying_sound: Res<EnemyDyingSound>,
) {
    for event in enemy_events.iter() {
        match event {
            EnemyEvents::Kill(_) => {
                audio.play(enemy_dying_sound.0.clone());
            }
            _ => {} // todo add sound for Spawn
        };
    }
}
