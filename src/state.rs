use bevy::prelude::*;
use iyes_loopless::prelude::*;

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub enum GameState {
    HomePage,
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

fn start_game(mut commands: Commands) {
    commands.insert_resource(NextState(GameState::Playing))
}

fn pause_game(mut commands: Commands) {
    commands.insert_resource(NextState(GameState::Pause))
}

fn resume_game(mut commands: Commands) {
    commands.insert_resource(NextState(GameState::Playing))
}

fn on_click_page(
    gamestate: Res<CurrentState<GameState>>,
    buttons: Res<Input<MouseButton>>,
    commands: Commands,
) {
    if gamestate.0 == GameState::HomePage && buttons.just_pressed(MouseButton::Left) {
        return start_game(commands);
    }
    if gamestate.0 == GameState::Playing && buttons.just_pressed(MouseButton::Left) {
        return pause_game(commands);
    }
    if gamestate.0 == GameState::Pause && buttons.just_pressed(MouseButton::Left) {
        return resume_game(commands);
    }
}
