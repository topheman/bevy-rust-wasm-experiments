use bevy::prelude::*;

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app.add_plugins(bevy_inspector_egui::bevy_egui::EguiPlugin::default())
                .add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new());
        }
    }
}
