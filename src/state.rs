use bevy::prelude::*;
use iyes_loopless::prelude::*;

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub enum GameState {
    HomePage,
    PrepareGame,
    Playing,
    Pause,
    Stopped(Box<GameState>),
    // GameOver,
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(GameState::HomePage);
        // .add_system(on_click_page);
    }
}

pub fn new_game(mut commands: Commands, gamestate: Res<CurrentState<GameState>>) {
    println!("call new_game");
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
    println!("call start_game");
    if gamestate.0 == GameState::PrepareGame {
        commands.insert_resource(NextState(GameState::Playing))
    } else {
        println!("Impossible state, you can only run start_game in GameState::PrepareGame state, ran from {:?}", gamestate.0);
    }
}

pub fn pause_game(mut commands: Commands, gamestate: Res<CurrentState<GameState>>) {
    println!("call pause_game");
    if gamestate.0 == GameState::Playing {
        commands.insert_resource(NextState(GameState::Pause))
    } else {
        println!("Impossible state, you can only run pause_game in GameState::Playing state, ran from {:?}", gamestate.0);
    }
}

pub fn resume_game(mut commands: Commands, gamestate: Res<CurrentState<GameState>>) {
    println!("call resume_game");
    if gamestate.0 == GameState::Pause {
        commands.insert_resource(NextState(GameState::Playing))
    } else {
        println!(
            "Impossible state, you can only run resume_game in GameState::Pause state, ran from {:?}",
            gamestate.0
        );
    }
}

pub fn stop_loop(mut commands: Commands, gamestate: Res<CurrentState<GameState>>) {
    println!("call stop_loop");
    if gamestate.0 == GameState::Playing {
        commands.insert_resource(NextState(GameState::Stopped(Box::new(GameState::Playing))))
    } else if gamestate.0 == GameState::Pause {
        commands.insert_resource(NextState(GameState::Stopped(Box::new(GameState::Pause))))
    } else {
        println!("Impossible state, you can only run stop_loop in GameState::{{Playing,Pause}} state, ran from {:?}", gamestate.0);
    }
}

pub fn resume_loop(mut commands: Commands, gamestate: Res<CurrentState<GameState>>) {
    println!("call resume_loop");
    match &gamestate.0 {
        GameState::Stopped(boxed_prev_state) => {
            let unboxed_prev_state = *(*boxed_prev_state).clone();
            commands.insert_resource(NextState(unboxed_prev_state))
        }
        _ => {
            println!("Impossible state, you can only run resume_loop in GameState::Stopped state, ran from {:?}", gamestate.0);
        }
    }
}

// pub fn game_over(mut commands: Commands, gamestate: Res<CurrentState<GameState>>) {
//     println!("call game_over");
//     if gamestate.0 == GameState::Playing {
//         commands.insert_resource(NextState(GameState::GameOver))
//     } else {
//         println!(
//             "Impossible state, you can only run game_over in GameState::Playing state, ran from {:?}",
//             gamestate.0
//         );
//     }
// }

// fn on_click_page(
//     gamestate: Res<CurrentState<GameState>>,
//     buttons: Res<Input<MouseButton>>,
//     commands: Commands,
// ) {
//     if gamestate.0 == GameState::HomePage && buttons.just_pressed(MouseButton::Left) {
//         return new_game(commands, gamestate);
//     }
//     if gamestate.0 == GameState::PrepareGame && buttons.just_pressed(MouseButton::Left) {
//         return start_game(commands, gamestate);
//     }
//     if gamestate.0 == GameState::Playing && buttons.just_pressed(MouseButton::Left) {
//         return pause_game(commands, gamestate);
//     }
//     if gamestate.0 == GameState::Pause && buttons.just_released(MouseButton::Left) {
//         return resume_game(commands, gamestate);
//     }
// }
