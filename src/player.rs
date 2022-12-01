use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

pub const PLAYER_SCALE: f32 = 1.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_startup_system_to_stage(StartupStage::PreStartup, load_assets)
            .add_system(player_movement.label("movement"));
    }
}

#[derive(Resource)]
struct BallTexture(Handle<TextureAtlas>);

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("ball-steel-no-shadow.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(100.0, 110.0),
        7,
        1,
        None,
        Some(Vec2::new(-100.0, 0.0)),
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.insert_resource(BallTexture(texture_atlas_handle));
}

#[derive(Component, Inspectable)]
pub struct Player {
    speed: f32,
}

fn player_movement(
    mut player_query: Query<(&Player, &mut Transform)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (player, mut transform) = player_query.single_mut();

    if keyboard.pressed(KeyCode::Up) {
        transform.translation.y += 1.0 * player.speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::Down) {
        transform.translation.y -= 1.0 * player.speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::Left) {
        transform.translation.x -= 1.0 * player.speed * time.delta_seconds();
    }
    if keyboard.pressed(KeyCode::Right) {
        transform.translation.x += 1.0 * player.speed * time.delta_seconds();
    }
}

fn spawn_player(mut commands: Commands, ball_texture: Res<BallTexture>) {
    let mut sprite = TextureAtlasSprite::new(1);
    sprite.color = Color::rgb(0.4, 0.9, 0.9);

    commands
        .spawn(SpriteSheetBundle {
            sprite,
            texture_atlas: ball_texture.0.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 900.0), // 900.0 is like z-index
                scale: Vec3::splat(PLAYER_SCALE),
                ..default()
            },
            ..default()
        })
        .insert(Name::new("Player"))
        .insert(Player { speed: 200.0 });
}
