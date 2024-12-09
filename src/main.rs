use bevy::prelude::*;
use crate::environment::EnvironmentPlugin;
use crate::player::PlayerPlugin;

mod environment;
mod player;
mod constants;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .add_plugins(EnvironmentPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new())
        .run();
}

fn startup() {
}