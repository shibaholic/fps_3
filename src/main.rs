use avian3d::PhysicsPlugins;
use bevy::prelude::*;
use bevy::window::CursorGrabMode;
use player::component::LogicalPlayerController;
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

        .register_type::<CursorLocked>()
        .insert_resource(CursorLocked { 0:false })
        .add_systems(Update, manage_cursor)

        .run();
}

#[derive(Resource, PartialEq, Reflect)]
pub struct CursorLocked(bool);

fn manage_cursor(
    btn: Res<ButtonInput<MouseButton>>,
    key: Res<ButtonInput<KeyCode>>,
    mut window_query: Query<&mut Window>,
    mut cursor_lock: ResMut<CursorLocked>,
) {
    for mut window in &mut window_query {
        if btn.just_pressed(MouseButton::Left) {
            window.cursor_options.grab_mode = CursorGrabMode::Locked;
            window.cursor_options.visible = false;
            cursor_lock.0 = true;
        }
        if key.just_pressed(KeyCode::Escape) {
            window.cursor_options.grab_mode = CursorGrabMode::None;
            window.cursor_options.visible = true;
            cursor_lock.0 = false;
        }
    }
}