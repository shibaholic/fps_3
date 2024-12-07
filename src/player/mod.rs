use bevy::{input::mouse::AccumulatedMouseMotion, prelude::*};

use std::f32::consts::FRAC_PI_2;

const ANGLE_EPSILON: f32 = 0.001953125;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, spawn_player)
        .add_systems(Update, (player_input, player_look, player_move, player_render))
        ;
    }
}

// Logical player component flag
#[derive(Component)]
pub struct LogicalPlayer;

// Keybindings and control settings
#[derive(Component)]
pub struct PlayerControls {
    // put keys inside here
    pub mouse_sensitivity:f32
}

impl Default for PlayerControls {
    fn default() -> Self {
        Self {
            mouse_sensitivity: 0.001
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

// Contains the data like position and view angle
#[derive(Component, Default)]
pub struct LogicalPlayerController {
    pub pitch: f32,
    pub yaw: f32,
}

// Render player component flag and parent to LogicalPlayer entity
#[derive(Component)]
pub struct RenderPlayer {
    pub logical_entity: Entity,
}

fn spawn_player(
    mut commands: Commands
) {
    // logical player entity
    let logical_player = commands.spawn((
        Transform::from_xyz(0.0, 4.0, 0.0),
        LogicalPlayer,
        LogicalPlayerController::default(),
        PlayerControls::default(),
        PlayerInput::default()
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

// transforms raw input into PlayerInput
fn player_input(
    mouse_motion: Res<AccumulatedMouseMotion>,
    mut query: Query<(&mut PlayerInput, &PlayerControls)>
) {
    let Ok((mut player_input, player_controls)) = query.get_single_mut() else {
        return;
    };

    let delta = mouse_motion.delta;

    println!("[player_input] delta: {} {}", delta.x, delta.y);

    if delta != Vec2::ZERO {
        
        let delta_yaw = -delta.x * player_controls.mouse_sensitivity;
        let delta_pitch = -delta.y * player_controls.mouse_sensitivity;

        player_input.yaw = delta_yaw;
        player_input.pitch = delta_pitch;
    } else {
        player_input.yaw = 0.0;
        player_input.pitch = 0.0;
    }
}

// transforms PlayerInput into LogicPlayerData for look only
// since PlayerInput pitch and yaw is delta, then we need to add it to the current yaw and pitch in logical_controller
fn player_look(
    mut query: Query<(&mut LogicalPlayerController, &PlayerInput)>
) {
    let Ok((mut logical_controller, player_input)) = query.get_single_mut() else {
        return;
    };

    logical_controller.yaw += player_input.yaw;

    const PITCH_LIMIT:f32 = FRAC_PI_2 - ANGLE_EPSILON;

    logical_controller.pitch = (logical_controller.pitch + player_input.pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);
}

// transforms PlayerInput + LogicPlayerData (look) into LogicPlayerData for move
fn player_move(

) {

}

// render the LogicPlayerData by transfering logic to render_player
fn player_render(
    mut render_query: Query<(&mut Transform, &RenderPlayer), With<RenderPlayer>>,
    logical_query: Query<(&Transform, &LogicalPlayerController), (With<LogicalPlayer>, Without<RenderPlayer>)>
) {

    let Ok((mut render_transform, render_player)) = render_query.get_single_mut() else {
        return;
    };

    let Ok((logical_transform, logical_controller)) = logical_query.get(render_player.logical_entity) else {
        return;
    };

    // TODO: use logical_collider as height for camera
    let camera_offset = 1.0;

    render_transform.translation = logical_transform.translation + camera_offset;
    render_transform.rotation = Quat::from_euler(EulerRot::YXZ, logical_controller.yaw, logical_controller.pitch, 0.0);

}
