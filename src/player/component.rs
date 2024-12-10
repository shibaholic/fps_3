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
    pub key_down: KeyCode,

    pub key_fly: KeyCode,
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
            key_down: KeyCode::KeyE,

            key_fly: KeyCode::KeyF
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
    pub fly_speed: Scalar,
    pub gravity: Vector,
    pub damping_factor: Scalar,
}

impl Default for LogicalPlayerProperties {
    fn default() -> Self {
        Self {
            fly_speed: 10.0,
            gravity: Vector::NEG_Y * 9.81 * 0.1,
            damping_factor: 0.92
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
