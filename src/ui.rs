/**
 * The code in that module contains:
 * - the ui
 * - the interaction with the ui
 *
 * If you have a better way to do it, you are welcome to open a PR.
 *
 * You can't beat HTML, jsx or templating languages for ui ...
 */
use bevy::prelude::*;
use bevy::window::{CursorIcon, PrimaryWindow, SystemCursorIcon};

use crate::colors::DEFAULT_COLOR;
use crate::state::{new_game, pause_game, resume_game, GameState};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::HomePage), (title, top_menu, home_and_pause, home))
            .add_systems(OnExit(GameState::HomePage), (despawn_with::<MainMenu>, despawn_with::<Title>, despawn_with::<Welcome>))
            .add_systems(OnEnter(GameState::Pause), (home_and_pause, pause, title))
            .add_systems(OnExit(GameState::Pause), (despawn_with::<MainMenu>, despawn_with::<Title>, despawn_with::<Welcome>))
            .add_systems(Update, (
                handle_keyboard_pause,
                handle_main_btn_click,
                handle_pause_btn_click,
                handle_pause_btn_hover,
                handle_main_btn_hover,
            ));
    }
}

#[derive(Component)]
struct MainMenu;

#[derive(Component)]
pub struct MainButton;

#[derive(Component)]
struct Title;

#[derive(Component)]
struct Welcome;

#[derive(Component)]
struct TopMenu;

#[derive(Component)]
pub struct PauseButton;

/// Despawn all entities with a given component type
fn despawn_with<T: Component>(mut commands: Commands, q: Query<Entity, With<T>>) {
    for e in q.iter() {
        commands.entity(e).despawn();
    }
}

fn top_menu(mut commands: Commands, ass: Res<AssetServer>) {
    let butt_style = Node {
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        padding: UiRect::all(Val::Px(8.0)),
        margin: UiRect::all(Val::Px(4.0)),
        border: UiRect::all(Val::Px(2.0)),
        flex_grow: 1.0,
        ..Default::default()
    };

    let top_menu = commands
        .spawn((
            Node {
                width: Val::Auto,
                height: Val::Auto,
                margin: UiRect::all(Val::Auto),
                align_self: AlignSelf::Center,
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::Center,
                position_type: PositionType::Absolute,
                top: Val::Px(15.0),
                left: Val::Px(15.0),
                ..Default::default()
            },
            TopMenu,
        ))
        .id();

    let pause_button = commands
        .spawn((
            Button,
            butt_style.clone(),
            BackgroundColor(Color::WHITE),
            BorderColor::all(Color::srgb(0.5, 0.5, 0.5)),
            PauseButton,
        ))
        .with_children(|btn| {
            btn.spawn((
                Text::new("P"),
                TextFont {
                    font: ass.load("ThaleahFat.ttf"),
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::BLACK),
            ));
        })
        .id();

    commands.entity(top_menu).add_children(&[pause_button]);
}

fn instuctions<'a, 'b>(mut commands: Commands<'a, 'b>, ass: &Res<AssetServer>) -> Commands<'a, 'b> {
    commands.spawn((
        Text::new("P: pause\n\nSPACE/TAP: add a ball to bounce with\n(min 2 balls / max 10 balls)\n\nARROW keys: move the ball\n\nOn mobile, tilt your device"),
        TextFont {
            font: ass.load("m6x11.ttf"),
            font_size: 20.0,
            ..default()
        },
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(Justify::Left),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(80.0),
            left: Val::Px(15.0),
            ..default()
        },
        Welcome,
    ));
    return commands;
}

fn make_button<'a, 'b>(
    content: &str,
    mut commands: Commands<'a, 'b>,
    ass: &Res<AssetServer>,
) -> Commands<'a, 'b> {
    let butt_style = Node {
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        padding: UiRect::all(Val::Px(8.0)),
        margin: UiRect::all(Val::Px(4.0)),
        border: UiRect::all(Val::Px(2.0)),
        flex_grow: 1.0,
        ..Default::default()
    };

    let menu = commands
        .spawn((
            Node {
                width: Val::Auto,
                height: Val::Auto,
                margin: UiRect::all(Val::Auto),
                align_self: AlignSelf::Center,
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            MainMenu,
        ))
        .id();

    let start_game_button = commands
        .spawn((
            Button,
            butt_style.clone(),
            BackgroundColor(Color::WHITE),
            BorderColor::all(Color::srgb(0.5, 0.5, 0.5)),
            MainButton,
        ))
        .with_children(|btn| {
            btn.spawn((
                Text::new(content),
                TextFont {
                    font: ass.load("ThaleahFat.ttf"),
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::BLACK),
            ));
        })
        .id();

    commands.entity(menu).add_children(&[start_game_button]);

    return commands;
}

fn title(mut commands: Commands, ass: Res<AssetServer>) {
    commands.spawn((
        Text::new("bevy-rust-\nwasm-experiments"),
        TextFont {
            font: ass.load("ThaleahFat.ttf"),
            font_size: 35.0,
            ..default()
        },
        TextColor(Color::WHITE),
        TextLayout::new_with_justify(Justify::Center),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(15.0),
            ..default()
        },
        Title,
    ));
}

fn home_and_pause(commands: Commands, ass: Res<AssetServer>) {
    instuctions(commands, &ass);
}

fn pause(commands: Commands, ass: Res<AssetServer>, mut camera_query: Query<&mut Camera, With<Camera2d>>) {
    let mut camera = camera_query.single_mut().unwrap();
    camera.clear_color = ClearColorConfig::Custom(DEFAULT_COLOR);
    make_button("Pause", commands, &ass);
}

fn home(commands: Commands, ass: Res<AssetServer>) {
    make_button("Start", commands, &ass);
}

/**
 * Following handlers for click/tap.
 *
 * For the moment, `Interaction::Released` is not yet available: https://github.com/bevyengine/bevy/issues/5769
 * so I didn't out the resume_game on the pause button
 */

fn handle_main_btn_click(
    query: Query<&Interaction, With<MainButton>>,
    gamestate: Res<State<GameState>>,
    commands: Commands,
) {
    if let Ok(interaction) = query.single() {
        match interaction {
            Interaction::Pressed => {
                if *gamestate == GameState::HomePage {
                    new_game(commands, gamestate);
                } else if *gamestate == GameState::Pause {
                    resume_game(commands, gamestate);
                }
            }
            _ => {}
        }
    }
}

fn handle_pause_btn_click(
    query: Query<&Interaction, With<PauseButton>>,
    gamestate: Res<State<GameState>>,
    commands: Commands,
) {
    if let Ok(interaction) = query.single() {
        match interaction {
            Interaction::Pressed => {
                if *gamestate == GameState::Playing {
                    pause_game(commands, gamestate);
                }
            }
            _ => {}
        }
    }
}

fn handle_pause_btn_hover(
    query: Query<&Interaction, With<PauseButton>>,
    mut window_query: Query<&mut CursorIcon, With<PrimaryWindow>>,
) {
    if let Ok(interaction) = query.single() {
        match interaction {
            Interaction::Hovered => {
                if let Ok(mut cursor) = window_query.single_mut() {
                    *cursor = CursorIcon::System(SystemCursorIcon::Pointer);
                }
            }
            _ => {}
        }
    }
}

fn handle_main_btn_hover(
    query: Query<&Interaction, With<MainButton>>,
    mut window_query: Query<&mut CursorIcon, With<PrimaryWindow>>,
) {
    if let Ok(interaction) = query.single() {
        match interaction {
            Interaction::Hovered => {
                if let Ok(mut cursor) = window_query.single_mut() {
                    *cursor = CursorIcon::System(SystemCursorIcon::Pointer);
                }
            }
            _ => {}
        }
    }
}

fn handle_keyboard_pause(
    commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    gamestate: Res<State<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::KeyP) {
        if *gamestate == GameState::Playing {
            pause_game(commands, gamestate);
        } else if *gamestate == GameState::Pause {
            resume_game(commands, gamestate);
        }
    }
}
