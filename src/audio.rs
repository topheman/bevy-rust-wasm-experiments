use bevy::prelude::*;

use crate::ball::CollisionEvent;
use crate::enemies::EnemyEvents;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, load_sounds)
            .add_systems(Update, (play_enemy_sounds, play_collision_sounds));
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
    mut commands: Commands,
    mut collision_events: MessageReader<CollisionEvent>,
    ball_wall_sound: Res<BallWallSound>,
) {
    for event in collision_events.read() {
        match event {
            CollisionEvent::BallWall(_) => {
                commands.spawn((
                    AudioPlayer::new(ball_wall_sound.0.clone()),
                    PlaybackSettings::DESPAWN,
                ));
            }
            _ => {
                commands.spawn((
                    AudioPlayer::new(ball_wall_sound.0.clone()),
                    PlaybackSettings::DESPAWN,
                ));
            }
        };
    }
}

fn play_enemy_sounds(
    mut commands: Commands,
    mut enemy_events: MessageReader<EnemyEvents>,
    enemy_dying_sound: Res<EnemyDyingSound>,
) {
    for event in enemy_events.read() {
        match event {
            EnemyEvents::Kill(_) => {
                commands.spawn((
                    AudioPlayer::new(enemy_dying_sound.0.clone()),
                    PlaybackSettings::DESPAWN,
                ));
            }
            _ => {}
        };
    }
}
