use avian3d::{math::*, prelude::*};
use bevy::{input::mouse::MouseMotion, prelude::*};

use std::f32::consts::FRAC_PI_2;

use crate::{constants::*, CursorLocked};
use super::component::{Grounded, LogicalPlayer, LogicalPlayerController, LogicalPlayerProperties, MoveMode, PlayerControls, PlayerInput, RenderPlayer};

// transforms raw input into PlayerInput
pub fn player_input(
    mut mouse_events: EventReader<MouseMotion>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut PlayerInput, &PlayerControls)>,
    cursor_locked: Res<CursorLocked>
) {
    let Ok((mut player_input, player_controls)) = query.get_single_mut() else {
        return;
    };

    if !cursor_locked.0 {
        *player_input = PlayerInput::default();
        return;
    }

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
        get_axis(&keyboard_input, player_controls.key_right, player_controls.key_left),
        get_axis(&keyboard_input, player_controls.key_up, player_controls.key_down),
        get_axis(&keyboard_input, player_controls.key_forward, player_controls.key_backward)
    );

    // TODO: jump, crouch
    player_input.fly = keyboard_input.just_pressed(player_controls.key_fly);
    player_input.jump = keyboard_input.just_pressed(player_controls.key_jump);
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
/// Updates the [`Grounded`] status for character controllers.
pub fn update_grounded(
    mut commands: Commands,
    mut query: Query<(Entity, &ShapeHits, &Rotation, &LogicalPlayerProperties)>,
) {
    for (entity, hits, rotation, player_props) in &mut query {
        // The character is grounded if the shape caster has a hit with a normal
        // that isn't too steep.
        let is_grounded = hits.iter().any(|hit| {
            (rotation * -hit.normal2).angle_between(Vector::Y).abs() <= player_props.max_slope_angle
        });

        info!("update grounded");

        if is_grounded {
            info!("add grounded");
            commands.entity(entity).insert(Grounded);
        } else {
            info!("remove grounded");
            commands.entity(entity).remove::<Grounded>();
        }
    }
}

// transforms PlayerInput + a little LogicPlayerController (look) into LogicPlayerController (move)
pub fn player_move(
    time: Res<Time>,
    mut query: Query<(&mut GravityScale, &PlayerInput, &LogicalPlayerProperties, &mut LogicalPlayerController, &mut LinearVelocity, Has<Grounded>)>
) {
    let Ok((mut gravity_scale, 
        player_input, 
        player_props, 
        mut logical_controller, 
        mut linear_velocity,
        grounded)) = 
    query.get_single_mut() else {
        return;
    };

    let delta_time = time.delta_secs();

    if player_input.fly {
        logical_controller.move_mode = match logical_controller.move_mode {
            MoveMode::Noclip => {
                gravity_scale.0 = 1.0;
                MoveMode::Ground
            },
            MoveMode::Ground => {
                gravity_scale.0 = 0.0;
                MoveMode::Noclip
            }
        }
    }

    if logical_controller.move_mode == MoveMode::Noclip {
        let mut move_to_world = Mat3::from_euler(EulerRot::YXZ, logical_controller.yaw, logical_controller.pitch, 0.0);
        move_to_world.z_axis *= -1.0; // Forward is -Z
        move_to_world.y_axis = Vec3::Y; // Up is Y
        linear_velocity.0 = move_to_world * player_input.movement * player_props.fly_velocity;

    } else if logical_controller.move_mode == MoveMode::Ground {
        let mut move_to_world = Mat3::from_euler(EulerRot::YXZ, logical_controller.yaw, 0.0, 0.0);
        move_to_world.z_axis *= -1.0; // Forward is -Z
        linear_velocity.0 += move_to_world * player_input.movement.with_y(0.0) * player_props.walk_accel * delta_time;

        if player_input.jump && grounded {
            linear_velocity.y += player_props.jump_impulse;
        }
    }
}

pub fn player_movement_damping(mut query: Query<(&LogicalPlayerProperties, &mut LinearVelocity)>) {
    for (player_props, mut linear_velocity) in &mut query {
        // We could use `LinearDamping`, but we don't want to dampen movement along the Y axis
        linear_velocity.x *= player_props.damping_factor;
        linear_velocity.z *= player_props.damping_factor;
    }
}

// render the LogicPlayerData by transfering logic to render_player
pub fn player_render(
    mut render_query: Query<(&mut Transform, &RenderPlayer), With<RenderPlayer>>,
    logical_query: Query<(&Transform, &LogicalPlayerController, &Collider), (With<LogicalPlayer>, Without<RenderPlayer>)>
) {

    let Ok((mut render_transform, render_player)) = render_query.get_single_mut() else {
        return;
    };

    let Ok((logical_transform, logical_controller, collider)) = logical_query.get(render_player.logical_entity) else {
        return;
    };

    let camera_offset = Vec3::Y * -0.5;
    let collider_offset = collider_y_offset(collider);

    render_transform.translation = logical_transform.translation + collider_offset + camera_offset;
    render_transform.rotation = Quat::from_euler(EulerRot::YXZ, logical_controller.yaw, logical_controller.pitch, 0.0);

}

/// Returns the offset that puts a point at the center of the player transform to the bottom of the collider.
/// Needed for when we want to originate something at the foot of the player.
fn collider_y_offset(collider: &Collider) -> Vec3 {
    Vec3::Y * if let Some(cylinder) = collider.shape().as_cylinder() {
        cylinder.half_height
    } else {
        panic!("Controller must use a cylinder collider")
    }
}