use bevy::{
    // diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    // render::render_resource::Texture,
    window::PresentMode,
};

// setup

pub const CLEAR_COLOR: Color = Color::rgb(1.0, 0.0, 0.0);
pub const PLAYER_SCALE: f32 = 1.0;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

// load resources

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

// setup player + texture

#[derive(Component)]
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

fn main() {
    App::new()
        .insert_resource(ClearColor(CLEAR_COLOR))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: 600.0,
                height: 600.0,
                title: "Bevy Rust Experiments".to_string(),
                resizable: false,
                cursor_visible: true,
                present_mode: PresentMode::AutoVsync,
                ..default()
            },
            ..default()
        }))
        .add_startup_system(setup)
        .add_startup_system(spawn_player)
        .add_startup_system_to_stage(StartupStage::PreStartup, load_assets)
        .add_system(player_movement.label("movement"))
        .run()
}
