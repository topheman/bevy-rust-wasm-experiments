use bevy::prelude::*;

pub struct TexturePlugin;

impl Plugin for TexturePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_assets);
    }
}

#[derive(Resource)]
pub struct BallTexture(Handle<TextureAtlas>);

pub fn spawn_assets_sprite(
    commands: &mut Commands,
    ball_texture: &BallTexture,
    index: usize, // set to 1
    color: Color,
    translation: Vec3,
    scale: Vec3,
) -> Entity {
    let mut sprite = TextureAtlasSprite::new(index);
    sprite.color = color;

    return commands
        .spawn(SpriteSheetBundle {
            sprite,
            texture_atlas: ball_texture.0.clone(),
            transform: Transform {
                translation,
                scale,
                ..default()
            },
            ..default()
        })
        .id();
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("ball-steel-no-shadow.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(100.0, 100.0),
        7,
        1,
        None,
        Some(Vec2::new(-100.0, 0.0)),
    );
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.insert_resource(BallTexture(texture_atlas_handle));
}
