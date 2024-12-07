use bevy::prelude::*;
use my_crate::environment::EnvironmentPlugin;
use my_crate::player::PlayerPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, startup)
        .add_plugins(EnvironmentPlugin)
        .add_plugins(PlayerPlugin)
        .run();
}

fn startup() {
}