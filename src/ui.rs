use std::borrow::{Borrow, BorrowMut};

use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};
use iyes_loopless::prelude::*;

use crate::state::GameState;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::HomePage, home_page)
            .add_exit_system(GameState::HomePage, despawn_with::<MainMenu>)
            .add_exit_system(GameState::HomePage, despawn_with::<Title>)
            // .add_exit_system(GameState::HomePage, despawn_with::<Welcome>)
            .add_enter_system(GameState::Playing, playing)
            .add_enter_system(GameState::Pause, pause)
            .add_exit_system(GameState::Pause, despawn_with::<MainMenu>);
    }
}

#[derive(Component)]
struct MainMenu;

#[derive(Component)]
struct GameButton;

#[derive(Component)]
struct Title;

#[derive(Component)]
struct Welcome;

/// Despawn all entities with a given component type
fn despawn_with<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    for e in q.iter() {
        commands.entity(e).despawn_recursive();
    }
}

fn instuctions<'a, 'b>(mut commands: Commands<'a, 'b>, ass: &Res<AssetServer>) -> Commands<'a, 'b> {
    // welcome
    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "H: back to home page\nP: pause\nSPACE/TAP: slow down the ball\nARROW keys: move the ball\n\nOn mobile, tilt your device",
            TextStyle {
                font: ass.load("m6x11.ttf"),
                font_size: 24.0,
                color: Color::WHITE,
            },
        ) // Set the alignment of the Text
        .with_text_alignment(TextAlignment::TOP_LEFT)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(15.0),
                left: Val::Px(15.0),
                ..default()
            },
            ..default()
        }),
        Welcome,
    ));
    return commands;
}

fn make_button<'a, 'b>(
    content: &str,
    mut commands: Commands<'a, 'b>,
    ass: &Res<AssetServer>,
) -> Commands<'a, 'b> {
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
            GameButton,
        ))
        .with_children(|btn| {
            btn.spawn(TextBundle {
                text: Text::from_section(content, butt_textstyle.clone()),
                ..Default::default()
            });
        })
        .id();

    commands.entity(menu).push_children(&[start_game_button]);

    return commands;
}

fn home_page(mut query: Query<&mut Camera2d>, mut commands: Commands, ass: Res<AssetServer>) {
    println!("home_page");
    let mut camera = query.single_mut();
    camera.clear_color = ClearColorConfig::Custom(Color::rgb(1.0, 0.0, 0.0));

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

    let mut commands2 = instuctions(commands, &ass);

    make_button("Start Game", commands2, &ass);
}

fn playing(mut query: Query<&mut Camera2d>) {
    println!("playing");
    let mut camera = query.single_mut();
    camera.clear_color = ClearColorConfig::Custom(Color::rgb(0.0, 0.5, 0.0));
}

fn pause(mut query: Query<&mut Camera2d>, mut commands: Commands, ass: Res<AssetServer>) {
    println!("pause");
    let mut camera = query.single_mut();
    camera.clear_color = ClearColorConfig::Custom(Color::rgb(0.5, 0.0, 0.5));
    make_button("Pause", commands, &ass);
}
