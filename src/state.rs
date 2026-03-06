use bevy::prelude::*;

#[derive(States, Clone, Hash, Eq, PartialEq, Debug, Default)]
pub enum GameState {
    #[default]
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
        app.init_state::<GameState>();
    }
}

pub fn new_game(mut commands: Commands, gamestate: Res<State<GameState>>) {
    if *gamestate == GameState::HomePage {
        commands.insert_resource(NextState::Pending(GameState::PrepareGame));
    } else {
        println!(
            "Impossible state, you can only run new_game in GameState::HomePage state, ran from {:?}",
            gamestate.get()
        );
    }
}

pub fn start_game(mut commands: Commands, gamestate: Res<State<GameState>>) {
    if *gamestate == GameState::PrepareGame {
        commands.insert_resource(NextState::Pending(GameState::Playing))
    } else {
        println!("Impossible state, you can only run start_game in GameState::PrepareGame state, ran from {:?}", gamestate.get());
    }
}

pub fn pause_game(mut commands: Commands, gamestate: Res<State<GameState>>) {
    if *gamestate == GameState::Playing {
        commands.insert_resource(NextState::Pending(GameState::Pause))
    } else {
        println!("Impossible state, you can only run pause_game in GameState::Playing state, ran from {:?}", gamestate.get());
    }
}

pub fn resume_game(mut commands: Commands, gamestate: Res<State<GameState>>) {
    if *gamestate == GameState::Pause {
        commands.insert_resource(NextState::Pending(GameState::Playing))
    } else {
        println!(
            "Impossible state, you can only run resume_game in GameState::Pause state, ran from {:?}",
            gamestate.get()
        );
    }
}

pub fn stop_loop(mut commands: Commands, gamestate: Res<State<GameState>>) {
    if *gamestate == GameState::Playing {
        commands.insert_resource(NextState::Pending(GameState::Stopped(Box::new(GameState::Playing))))
    } else if *gamestate == GameState::Pause {
        commands.insert_resource(NextState::Pending(GameState::Stopped(Box::new(GameState::Pause))))
    } else {
        println!("Impossible state, you can only run stop_loop in GameState::{{Playing,Pause}} state, ran from {:?}", gamestate.get());
    }
}

pub fn resume_loop(mut commands: Commands, gamestate: Res<State<GameState>>) {
    match gamestate.get() {
        GameState::Stopped(boxed_prev_state) => {
            let unboxed_prev_state = *boxed_prev_state.clone();
            commands.insert_resource(NextState::Pending(unboxed_prev_state))
        }
        _ => {
            println!("Impossible state, you can only run resume_loop in GameState::Stopped state, ran from {:?}", gamestate.get());
        }
    }
}
