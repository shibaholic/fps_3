use avian3d::PhysicsPlugins;
use bevy::prelude::*;
use register::RegisterPlugin;
use crate::environment::EnvironmentPlugin;
use crate::player::PlayerPlugin;

mod environment;
mod player;
mod constants;
mod register;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(EnvironmentPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(bevy_inspector_egui::quick::WorldInspectorPlugin::new())
        .add_plugins(RegisterPlugin)
        .run();
}
