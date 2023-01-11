use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};
use iyes_loopless::prelude::*;

use crate::state::GameState;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::HomePage, home_page)
            .add_enter_system(GameState::Playing, playing)
            .add_enter_system(GameState::Pause, pause);
    }
}

fn home_page(mut query: Query<&mut Camera2d>) {
    println!("home_page");
    let mut camera = query.single_mut();
    camera.clear_color = ClearColorConfig::Custom(Color::rgb(1.0, 0.0, 0.0));
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
