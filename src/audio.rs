use bevy::prelude::*;

use crate::ball::CollisionEvent;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_sounds)
            .add_system(play_sounds);
    }
}

#[derive(Resource)]
struct BallWallSound(Handle<AudioSource>);

fn load_sounds(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ball_wall_sound = asset_server.load("impactMetal_heavy_004.ogg");
    commands.insert_resource(BallWallSound(ball_wall_sound));
}

fn play_sounds(
    mut collision_events: EventReader<CollisionEvent>,
    audio: Res<Audio>,
    ball_wall_sound: Res<BallWallSound>,
) {
    for event in collision_events.iter() {
        match event {
            CollisionEvent::BallWall => audio.play(ball_wall_sound.0.clone()),
            _ => audio.play(ball_wall_sound.0.clone()),
        };
    }
}
