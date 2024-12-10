use avian3d::{math::{Quaternion, Vector}, prelude::{CoefficientCombine, Collider, Friction, GravityScale, LockedAxes, Mass, Restitution, RigidBody, ShapeCaster, SleepingDisabled}};
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
        .add_systems(PreUpdate, (player_input, player_look,
            player_move, player_render
            ).chain()
        )
        ;
    }
}

fn spawn_player(
    mut commands: Commands
) {
    let height = 3.0;
    let collider = Collider::cylinder(0.5, height / 2.0);
    let mut caster_shape = collider.clone();
    caster_shape.set_scale(Vector::ONE * 0.99, 10);

    // logical player entity
    let logical_player = commands.spawn((
        Transform::from_xyz(0.0, 4.0, 0.0),
        LogicalPlayer,
        LogicalPlayerProperties::default(),
        LogicalPlayerController::default(),
        PlayerControls::default(),
        PlayerInput::default(),
        
        RigidBody::Dynamic,
        collider,
        GravityScale(0.0), // gravity is handled in player_move(), so surfing is supported.
        ShapeCaster::new(
            caster_shape,
            Vector::ZERO,
            Quaternion::default(),
            Dir3::NEG_Y,
        ).with_max_distance(0.2),
        LockedAxes::ROTATION_LOCKED,
        SleepingDisabled,
        Friction::ZERO.with_combine_rule(CoefficientCombine::Min),
        Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
        Mass(1.0)
        
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