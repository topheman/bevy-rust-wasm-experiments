use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};
use iyes_loopless::prelude::*;

use crate::state::GameState;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::HomePage, home_page)
            .add_exit_system(GameState::HomePage, despawn_with::<MainMenu>)
            .add_exit_system(GameState::HomePage, despawn_with::<Title>)
            .add_enter_system(GameState::Playing, playing)
            .add_enter_system(GameState::Pause, pause);
    }
}

#[derive(Component)]
struct MainMenu;

#[derive(Component)]
struct StartGameButton;

#[derive(Component)]
struct Title;

/// Despawn all entities with a given component type
fn despawn_with<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}

fn home_page(mut query: Query<&mut Camera2d>, mut commands: Commands, ass: Res<AssetServer>) {
    println!("home_page");
    let mut camera = query.single_mut();
    camera.clear_color = ClearColorConfig::Custom(Color::rgb(1.0, 0.0, 0.0));

    let butt_style = Style {
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        padding: UiRect::all(Val::Px(8.0)),
        margin: UiRect::all(Val::Px(4.0)),
        flex_grow: 1.0,
        ..Default::default()
    };
    let butt_textstyle = TextStyle {
        font: ass.load("ThaleahFat.ttf"),
        font_size: 24.0,
        color: Color::BLACK,
    };

    // title
    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "bevy-rust-\nwasm-experiments",
            TextStyle {
                font: ass.load("ThaleahFat.ttf"),
                font_size: 35.0,
                color: Color::WHITE,
            },
        ) // Set the alignment of the Text
        .with_text_alignment(TextAlignment::TOP_CENTER)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(5.0),
                right: Val::Px(15.0),
                ..default()
            },
            ..default()
        }),
        Title,
    ));

    let menu = commands
        .spawn((
            NodeBundle {
                background_color: BackgroundColor(Color::rgb(0.5, 0.5, 0.5)),
                style: Style {
                    size: Size::new(Val::Auto, Val::Auto),
                    margin: UiRect::all(Val::Auto),
                    align_self: AlignSelf::Center,
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                ..Default::default()
            },
            MainMenu,
        ))
        .id();

    let start_game_button = commands
        .spawn((
            ButtonBundle {
                style: butt_style.clone(),
                ..Default::default()
            },
            StartGameButton,
        ))
        .with_children(|btn| {
            btn.spawn(TextBundle {
                text: Text::from_section("Start Game", butt_textstyle.clone()),
                ..Default::default()
            });
        })
        .id();

    commands.entity(menu).push_children(&[start_game_button]);
}

fn playing(mut query: Query<&mut Camera2d>) {
    println!("playing");
    let mut camera = query.single_mut();
    camera.clear_color = ClearColorConfig::Custom(Color::rgb(0.0, 0.5, 0.0));
}

fn pause(mut query: Query<&mut Camera2d>) {
    println!("pause");
    let mut camera = query.single_mut();
    camera.clear_color = ClearColorConfig::Custom(Color::rgb(0.5, 0.0, 0.5));
}
