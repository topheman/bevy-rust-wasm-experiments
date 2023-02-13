use bevy::prelude::*;
use iyes_loopless::prelude::*;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::wasm_bindgen;

use crate::state::GameState;
use crate::state::{resume_loop, stop_loop};

pub struct StopLoopPlugin;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
extern "C" {
    fn is_stopped() -> bool;
}

#[cfg(not(target_arch = "wasm32"))]
fn is_stopped() -> bool {
    return false;
}

impl Plugin for StopLoopPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(pause_or_resume_loop);
    }
}

fn pause_or_resume_loop(commands: Commands, gamestate: Res<CurrentState<GameState>>) {
    let should_stop = is_stopped();
    match &gamestate.0 {
        GameState::Playing | GameState::Pause => {
            if should_stop {
                stop_loop(commands, gamestate)
            }
        }
        GameState::Stopped(_) => {
            if !should_stop {
                resume_loop(commands, gamestate)
            }
        }
        _ => {}
    }
}
