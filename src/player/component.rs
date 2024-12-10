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
    pub key_jump: KeyCode,
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

            key_fly: KeyCode::KeyF,
            key_jump: KeyCode::Space,
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
    pub fly_velocity: Scalar,
    pub walk_accel: Scalar,
    pub damping_factor: Scalar,
    pub jump_impulse: Scalar,
    pub max_slope_angle: Scalar,
    
    pub forward_speed: f32, // is this like the maximum speed?
    pub side_speed: f32,

    pub crouch_speed: f32,
    pub sprint_speed: f32,
    pub walk_speed: f32,

    pub traction_normal_cutoff: f32,
    pub friction_speed_cutoff: f32,
    pub stop_speed: f32,
    
    pub friction:f32,
    pub acceleration: f32,

    pub gravity:f32,

    pub air_speed_cap:f32,
    pub air_acceleration: f32,
    pub max_air_speed: f32,

}

impl Default for LogicalPlayerProperties {
    fn default() -> Self {
        Self {
            fly_velocity: 30.0,
            walk_accel: 30.0,
            damping_factor: 0.92,
            jump_impulse: 8.5,
            max_slope_angle: (30.0 as Scalar).to_radians(),

            forward_speed: 30.0,
            side_speed: 30.0,

            crouch_speed: 5.0,
            sprint_speed: 14.0,
            walk_speed: 9.0,

            traction_normal_cutoff: 0.7,
            friction_speed_cutoff: 0.1,
            stop_speed: 1.0,

            friction:10.0,
            acceleration: 10.0,

            gravity: 23.0,

            air_speed_cap: 2.0,
            air_acceleration: 20.0,
            max_air_speed: 15.0,
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
    pub ground_tick: u8,
}

// Render player component flag and parent to LogicalPlayer entity
#[derive(Component)]
pub struct RenderPlayer {
    pub logical_entity: Entity,
}
