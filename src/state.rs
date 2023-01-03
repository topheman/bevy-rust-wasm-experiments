use bevy::prelude::*;
use iyes_loopless::prelude::*;

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub enum GameState {
    LandingPage,
    Playing,
    Pause,
    GameOver,
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_loopless_state(GameState::LandingPage);
    }
}
