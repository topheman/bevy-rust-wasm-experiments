use bevy::prelude::*;

pub struct TexturePlugin;

impl Plugin for TexturePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, load_assets);
    }
}

#[derive(Resource)]
pub struct BallTexture {
    pub image: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
}

pub fn spawn_assets_sprite(
    commands: &mut Commands,
    ball_texture: &BallTexture,
    index: usize,
    color: Color,
    translation: Vec3,
    scale: Vec3,
) -> Entity {
    return commands
        .spawn((
            Sprite {
                image: ball_texture.image.clone(),
                color,
                texture_atlas: Some(TextureAtlas {
                    layout: ball_texture.layout.clone(),
                    index,
                }),
                ..default()
            },
            Transform {
                translation,
                scale,
                ..default()
            },
        ))
        .id();
}

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture_handle: Handle<Image> = asset_server.load("ball-steel-no-shadow.png");
    let layout = TextureAtlasLayout::from_grid(
        UVec2::new(100, 117),
        1,
        1,
        None,
        None,
    );
    let layout_handle = texture_atlas_layouts.add(layout);
    commands.insert_resource(BallTexture {
        image: texture_handle,
        layout: layout_handle,
    });
}
