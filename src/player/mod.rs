use avian3d::prelude::RigidBody;
use bevy::{prelude::*};

use component::{LogicalPlayer, LogicalPlayerController, LogicalPlayerProperties, PlayerControls, PlayerInput, RenderPlayer};
use system::{player_input, player_look, player_move, player_render};

pub mod system;
pub mod component;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, spawn_player)
        .add_systems(Update, (player_input, player_look, player_move, player_render).chain())
        ;
    }
}

fn spawn_player(
    mut commands: Commands
) {
    // logical player entity
    let logical_player = commands.spawn((
        Transform::from_xyz(0.0, 4.0, 0.0),
        LogicalPlayer,
        LogicalPlayerProperties::default(),
        LogicalPlayerController::default(),
        PlayerControls::default(),
        PlayerInput::default(),
        
        RigidBody::Kinematic,
        // Collider,
        
    ))
    .insert(Name::new("LogicalPlayer"))
    .id();

    // render player entity
    commands.spawn((
        Camera3d::default(),
        Projection::from(PerspectiveProjection {
            fov: 90.0_f32.to_radians(),
            ..default()
        }),
        Transform::from_xyz(0.0, 4.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
        RenderPlayer { logical_entity: logical_player }
    ))
    .insert(Name::new("RenderPlayer"));
}