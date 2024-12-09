use bevy::{input::mouse::MouseMotion, prelude::*};

use std::f32::consts::FRAC_PI_2;

use crate::constants::*;
use super::component::{LogicalPlayer, LogicalPlayerController, PlayerControls, PlayerInput, RenderPlayer, MoveMode};

// transforms raw input into PlayerInput
pub fn player_input(
    mut mouse_events: EventReader<MouseMotion>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut PlayerInput, &PlayerControls)>
) {
    let Ok((mut player_input, player_controls)) = query.get_single_mut() else {
        return;
    };

    // mouse motion

    let mut delta = Vec2::ZERO;
    for mouse_event in mouse_events.read() {
        delta += mouse_event.delta;
    }

    if delta != Vec2::ZERO {
        
        let delta_yaw = -delta.x * player_controls.mouse_sensitivity;
        let delta_pitch = -delta.y * player_controls.mouse_sensitivity;

        player_input.yaw = delta_yaw;
        player_input.pitch = delta_pitch;
    } else {
        player_input.yaw = 0.0;
        player_input.pitch = 0.0;
    }

    // keyboard

    fn get_axis(key_input: &Res<ButtonInput<KeyCode>>, key_pos: KeyCode, key_neg: KeyCode) -> f32 {
        get_pressed(key_input, key_pos) - get_pressed(key_input, key_neg)
    }

    fn get_pressed(key_input: &Res<ButtonInput<KeyCode>>, key: KeyCode) -> f32 {
        if key_input.pressed(key) {
            1.0
        } else {
            0.0
        }
    }

    player_input.movement = Vec3::new(
        get_axis(&keyboard_input, player_controls.key_left, player_controls.key_right),
        get_axis(&keyboard_input, player_controls.key_up, player_controls.key_down),
        get_axis(&keyboard_input, player_controls.key_forward, player_controls.key_backward)
    );

    // TODO: jump, crouch, fly
}

// transforms PlayerInput into LogicPlayerData for look only
// since PlayerInput pitch and yaw is delta, then we need to add it to the current yaw and pitch in logical_controller
pub fn player_look(
    mut query: Query<(&mut LogicalPlayerController, &PlayerInput)>
) {
    // CAREFUL: when it comes to controllers and view rotation, it might need to be delta_time'd.

    let Ok((mut logical_controller, player_input)) = query.get_single_mut() else {
        return;
    };

    logical_controller.yaw += player_input.yaw;

    const PITCH_LIMIT:f32 = FRAC_PI_2 - ANGLE_EPSILON;

    logical_controller.pitch = (logical_controller.pitch + player_input.pitch).clamp(-PITCH_LIMIT, PITCH_LIMIT);
}

// transforms PlayerInput + a little LogicPlayerController (look) into LogicPlayerController (move)
pub fn player_move(
    time: Res<Time>,
    mut query: Query<(&PlayerInput, &mut LogicalPlayerController)>
) {
    let Ok((player_input, mut logical_controller)) = query.get_single_mut() else {
        return;
    };

    // if player_input.fly {
    //     logical_controller.move_mode = match logical_controller.move_mode {
    //         MoveMode::Noclip => MoveMode::Ground,
    //         MoveMode::Ground => MoveMode::Noclip
    //     }
    // }

    if logical_controller.move_mode == MoveMode::Noclip {

    }
}

// render the LogicPlayerData by transfering logic to render_player
pub fn player_render(
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