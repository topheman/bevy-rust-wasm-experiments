use bevy::prelude::*;
use iyes_loopless::prelude::*;

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub enum GameState {
    HomePage,
    PrepareGame,
    Playing,
    Pause,
    GameOver,
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(GameState::HomePage)
            .add_system(on_click_page);
    }
}

pub fn new_game(mut commands: Commands, gamestate: Res<CurrentState<GameState>>) {
    if gamestate.0 == GameState::HomePage {
        commands.insert_resource(NextState(GameState::PrepareGame));
    } else {
        println!(
            "Impossible state, you can only run new_game in GameState::HomePage state, ran from {:?}",
            gamestate.0
        );
    }
}

pub fn start_game(mut commands: Commands, gamestate: Res<CurrentState<GameState>>) {
    if gamestate.0 == GameState::PrepareGame {
        commands.insert_resource(NextState(GameState::Playing))
    } else {
        println!("Impossible state, you can only run start_game in GameState::PrepareGame state, ran from {:?}", gamestate.0);
    }
}

pub fn pause_game(mut commands: Commands, gamestate: Res<CurrentState<GameState>>) {
    if gamestate.0 == GameState::Playing {
        commands.insert_resource(NextState(GameState::Pause))
    } else {
        println!("Impossible state, you can only run pause_game in GameState::Playing state, ran from {:?}", gamestate.0);
    }
}

pub fn resume_game(mut commands: Commands, gamestate: Res<CurrentState<GameState>>) {
    if gamestate.0 == GameState::Pause {
        commands.insert_resource(NextState(GameState::Playing))
    } else {
        println!(
            "Impossible state, you can only run resume_game in GameState::Pause state, ran from {:?}",
            gamestate.0
        );
    }
}

pub fn game_over(mut commands: Commands, gamestate: Res<CurrentState<GameState>>) {
    if gamestate.0 == GameState::Playing {
        commands.insert_resource(NextState(GameState::GameOver))
    } else {
        println!(
            "Impossible state, you can only run game_over in GameState::Playing state, ran from {:?}",
            gamestate.0
        );
    }
}

fn on_click_page(
    gamestate: Res<CurrentState<GameState>>,
    buttons: Res<Input<MouseButton>>,
    commands: Commands,
) {
    // if gamestate.0 == GameState::HomePage && buttons.just_pressed(MouseButton::Left) {
    //     return new_game(commands, gamestate);
    // }
    // if gamestate.0 == GameState::PrepareGame && buttons.just_pressed(MouseButton::Left) {
    //     return start_game(commands, gamestate);
    // }
    // if gamestate.0 == GameState::Playing && buttons.just_pressed(MouseButton::Left) {
    //     return pause_game(commands, gamestate);
    // }
    // if gamestate.0 == GameState::Pause && buttons.just_pressed(MouseButton::Left) {
    //     return resume_game(commands, gamestate);
    // }
}
