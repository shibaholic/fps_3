use avian3d::{math::*, prelude::*};
use bevy::prelude::*;

// Logical player component flag
#[derive(Component)]
pub struct LogicalPlayer;

// Keybindings and control settings
#[derive(Component)]
pub struct PlayerControls {
    // put keys inside here
    pub mouse_sensitivity:f32,
    pub key_left: KeyCode,
    pub key_right: KeyCode,
    pub key_forward: KeyCode,
    pub key_backward: KeyCode,
    pub key_up: KeyCode,
    pub key_down: KeyCode
}

impl Default for PlayerControls {
    fn default() -> Self {
        Self {
            mouse_sensitivity: 0.001,
            key_left: KeyCode::KeyA,
            key_right: KeyCode::KeyD,
            key_forward: KeyCode::KeyW,
            key_backward: KeyCode::KeyS,
            key_up: KeyCode::KeyQ,
            key_down: KeyCode::KeyE
        }
    }
}

// Not-raw player input
#[derive(Component, Default)]
pub struct PlayerInput {
    pub fly: bool,
    pub sprint: bool,
    pub jump: bool,
    pub crouch: bool,
    pub pitch: f32,
    pub yaw: f32,
    pub movement: Vec3,
}

#[derive(Component)]
pub struct LogicalPlayerProperties {
    pub fly_speed: f32
}

impl Default for LogicalPlayerProperties {
    fn default() -> Self {
        Self {
            fly_speed: 10.0
        }
    }
}

#[derive(PartialEq)]
pub enum MoveMode {
    Noclip,
    Ground,
}

impl Default for MoveMode {
    fn default() -> Self {
        MoveMode::Noclip
    }
}

// Contains physical state data about the logical player
// Not to be confused with LogicalPlayerProperties that contains speed, acceleration, friction values
#[derive(Component, Default)]
pub struct LogicalPlayerController {
    pub move_mode: MoveMode,
    
    pub pitch: f32,
    pub yaw: f32,
}

// Render player component flag and parent to LogicalPlayer entity
#[derive(Component)]
pub struct RenderPlayer {
    pub logical_entity: Entity,
}

// Avian3D Physics Bundles

/// The acceleration used for character movement.
#[derive(Component)]
pub struct MovementAcceleration(Scalar);

/// The damping factor used for slowing down movement.
#[derive(Component)]
pub struct MovementDampingFactor(Scalar);

/// The strength of a jump.
#[derive(Component)]
pub struct JumpImpulse(Scalar);

/// The gravitational acceleration used for a character controller.
#[derive(Component)]
pub struct ControllerGravity(Vector);

/// The maximum angle a slope can have for a character controller
/// to be able to climb and jump. If the slope is steeper than this angle,
/// the character will slide down.
#[derive(Component)]
pub struct MaxSlopeAngle(Scalar);

// A bundle to add kinematic physics to the player controller.
#[derive(Bundle)]
pub struct KinematicBundle {
    rigid_body: RigidBody,
    collider: Collider,
    ground_caster: ShapeCaster,
    gravity: ControllerGravity,
    movement: MovementBundle,
}

// A bundle that contains components for character movement.
#[derive(Bundle)]
pub struct MovementBundle {
    acceleration: MovementAcceleration,
    damping: MovementDampingFactor,
    jump_impulse: JumpImpulse,
    max_slope_angle: MaxSlopeAngle,
}