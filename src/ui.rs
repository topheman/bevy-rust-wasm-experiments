/**
 * The code in that module contains:
 * - the ui
 * - the interaction with the ui
 *
 * If you have a better way to do it, you are welcome to open a PR.
 *
 * You can't beat HTML, jsx or templating languages for ui ...
 */
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::colors::DEFAULT_COLOR;
use crate::state::{new_game, pause_game, resume_game, GameState};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::HomePage, title)
            .add_enter_system(GameState::HomePage, top_menu)
            .add_enter_system(GameState::HomePage, home_and_pause)
            .add_enter_system(GameState::HomePage, home)
            .add_exit_system(GameState::HomePage, despawn_with::<MainMenu>)
            .add_exit_system(GameState::HomePage, despawn_with::<Title>)
            .add_exit_system(GameState::HomePage, despawn_with::<Welcome>)
            .add_enter_system(GameState::Pause, home_and_pause)
            .add_enter_system(GameState::Pause, pause)
            .add_enter_system(GameState::Pause, title)
            .add_exit_system(GameState::Pause, despawn_with::<MainMenu>)
            .add_exit_system(GameState::Pause, despawn_with::<Title>)
            .add_exit_system(GameState::Pause, despawn_with::<Welcome>)
            .add_system(handle_keyboard_pause)
            .add_system(handle_main_btn_click)
            .add_system(handle_pause_btn_click)
            .add_system(handle_pause_btn_hover)
            .add_system(handle_main_btn_hover);
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
        commands.entity(e).despawn_recursive();
    }
}

fn top_menu(mut commands: Commands, ass: Res<AssetServer>) {
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

    let top_menu = commands
        .spawn((
            NodeBundle {
                background_color: BackgroundColor(Color::rgb(0.5, 0.5, 0.5)),
                style: Style {
                    size: Size::new(Val::Auto, Val::Auto),
                    margin: UiRect::all(Val::Auto),
                    align_self: AlignSelf::Center,
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        top: Val::Px(15.0),
                        left: Val::Px(15.0),
                        ..default()
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
            TopMenu,
        ))
        .id();

    let pause_button = commands
        .spawn((
            ButtonBundle {
                style: butt_style.clone(),
                ..Default::default()
            },
            PauseButton,
        ))
        .with_children(|btn| {
            btn.spawn(TextBundle {
                text: Text::from_section("P", butt_textstyle.clone()),
                ..Default::default()
            });
        })
        .id();

    commands.entity(top_menu).push_children(&[pause_button]);
}

fn instuctions<'a, 'b>(mut commands: Commands<'a, 'b>, ass: &Res<AssetServer>) -> Commands<'a, 'b> {
    // welcome
    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "P: pause\n\nSPACE/TAP: add a ball to bounce with\n(min 2 balls / max 10 balls)\n\nARROW keys: move the ball\n\nOn mobile, tilt your device",
            TextStyle {
                font: ass.load("m6x11.ttf"),
                font_size: 20.0,
                color: Color::WHITE,
            },
        ) // Set the alignment of the Text
        .with_text_alignment(TextAlignment::TOP_LEFT)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(80.0),
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
            MainButton,
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

fn title(mut commands: Commands, ass: Res<AssetServer>) {
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
}

fn home_and_pause(commands: Commands, ass: Res<AssetServer>) {
    instuctions(commands, &ass);
}

fn pause(commands: Commands, ass: Res<AssetServer>, mut camera_query: Query<&mut Camera2d>) {
    let mut camera = camera_query.single_mut();
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
    query: Query<(&Interaction, With<MainButton>)>,
    gamestate: Res<CurrentState<GameState>>,
    commands: Commands,
) {
    if let Ok((interaction, ())) = query.get_single() {
        match interaction {
            Interaction::Clicked => {
                if gamestate.0 == GameState::HomePage {
                    new_game(commands, gamestate);
                } else if gamestate.0 == GameState::Pause {
                    resume_game(commands, gamestate);
                }
            }
            _ => {}
        }
    }
}

fn handle_pause_btn_click(
    query: Query<(&Interaction, With<PauseButton>)>,
    gamestate: Res<CurrentState<GameState>>,
    commands: Commands,
) {
    if let Ok((interaction, ())) = query.get_single() {
        match interaction {
            Interaction::Clicked => {
                if gamestate.0 == GameState::Playing {
                    pause_game(commands, gamestate);
                }
            }
            _ => {}
        }
    }
}

fn handle_pause_btn_hover(
    query: Query<(&Interaction, With<PauseButton>)>,
    mut windows: ResMut<Windows>,
) {
    let window = windows.get_primary_mut().unwrap();
    let (interaction, ()) = query.single();
    match interaction {
        Interaction::Hovered => window.set_cursor_icon(CursorIcon::Hand),
        _ => {}
    }
}

fn handle_main_btn_hover(
    query: Query<(&Interaction, With<MainButton>)>,
    mut windows: ResMut<Windows>,
) {
    let window = windows.get_primary_mut().unwrap();
    if let Ok((interaction, ())) = query.get_single() {
        match interaction {
            Interaction::Hovered => window.set_cursor_icon(CursorIcon::Hand),
            _ => {}
        }
    }
}

fn handle_keyboard_pause(
    commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    gamestate: Res<CurrentState<GameState>>,
) {
    if keyboard.just_pressed(KeyCode::P) {
        if gamestate.0 == GameState::Playing {
            pause_game(commands, gamestate);
        } else if gamestate.0 == GameState::Pause {
            resume_game(commands, gamestate);
        }
    }
}
