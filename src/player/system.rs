use avian3d::{math::*, parry::query::ShapeCastHit, prelude::*};
use bevy::{ecs::query::QueryFilter, input::mouse::MouseMotion, prelude::*};

use std::f32::consts::FRAC_PI_2;

use crate::{constants::*, CursorLocked};
use super::component::{LogicalPlayer, LogicalPlayerController, LogicalPlayerProperties, MoveMode, PlayerControls, PlayerInput, RenderPlayer};

// If the distance to the ground is less than this value, the player is considered grounded
const GROUNDED_DISTANCE: f32 = 0.125;

const SLIGHT_SCALE_DOWN: f32 = 0.9375;

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

// transforms PlayerInput + a little LogicPlayerController (look) into LogicPlayerController (move)
pub fn player_move(
    time: Res<Time>,
    spatial_query: SpatialQuery,
    mut query: Query<(
        Entity, 
        &Transform,
        &Collider,
        &PlayerInput, 
        &LogicalPlayerProperties, 
        &mut LogicalPlayerController, 
        &mut LinearVelocity, 
    )>
) {
    let Ok((
        entity,
        transform,
        collider,
        player_input, 
        player_props, 
        mut logical_controller, 
        mut linear_velocity,)) = 
    query.get_single_mut() else {
        return;
    };

    let delta_time = time.delta_secs();

    if player_input.fly {
        logical_controller.move_mode = match logical_controller.move_mode {
            MoveMode::Noclip => {
                // gravity_scale.0 = 1.0;
                MoveMode::Ground
            },
            MoveMode::Ground => {
                // gravity_scale.0 = 0.0;
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
        // shape cast towards ground
        let filter = SpatialQueryFilter::default().with_excluded_entities([entity]);
        let config = ShapeCastConfig::from_max_distance(GROUNDED_DISTANCE);
        let ground_cast = spatial_query.cast_shape(
            &scaled_collider_laterally(&collider, SLIGHT_SCALE_DOWN),
            transform.translation,
            transform.rotation,
            -Dir3::Y,
            &config,
            &filter
        );

        // Source engine movement

        let speeds = Vec3::new(player_props.side_speed, 0.0, player_props.forward_speed);
        let mut move_to_world = Mat3::from_axis_angle(Vec3::Y, logical_controller.yaw);
        move_to_world.z_axis *= -1.0; // Forward is -Z
        let mut wish_direction = move_to_world * (player_input.movement * speeds); 
        let mut wish_speed = wish_direction.length();

        if wish_speed > f32::EPSILON {
            // avoid division by zero
            wish_direction /= wish_speed; // effectively normalizes to unit circle, avoiding length computation twice
        }

        // TODO: crouch and sprint speed
        let max_speed = player_props.sprint_speed;

        wish_speed = f32::min(wish_speed, max_speed); 

        if let Some(shape_hit_data) = ground_cast {
            // on the ground

            let has_traction = Vec3::dot(shape_hit_data.normal1, Vec3::Y) > player_props.traction_normal_cutoff;

            // only apply friction after at least one tick, allows b-hopping without losing speed
            if logical_controller.ground_tick >= 1 && has_traction {
                let lateral_speed = linear_velocity.xz().length();
                if lateral_speed > player_props.friction_speed_cutoff {
                    let control = f32::max(lateral_speed, player_props.stop_speed);
                    let drop = control * player_props.friction * delta_time;
                    let new_speed = f32::max((lateral_speed - drop) / lateral_speed, 0.0);
                    linear_velocity.x *= new_speed;
                    linear_velocity.z *= new_speed;
                } else {
                    linear_velocity.0 = Vec3::ZERO;
                }
                if logical_controller.ground_tick == 1 {
                    linear_velocity.y = 0.0; // for some reason, the other guy has something very weird
                }
            }

            let mut add = accelerate(
                wish_direction,
                wish_speed,
                player_props.acceleration,
                linear_velocity.0,
                delta_time,
            );
            if !has_traction { // basically turns off gravity if surfing right?
                info!("no traction");
                add.y -= player_props.gravity * delta_time;
            }
            linear_velocity.0 += add;

            if has_traction {
                info!("traction");
                let linear_velocity_2 = linear_velocity.0;
                // (how much current velocity aligns with hit_normal) * in the direction of hit_normal.
                linear_velocity.0 -= Vec3::dot(linear_velocity_2, shape_hit_data.normal1) * shape_hit_data.normal1;

                if player_input.jump {
                    info!("jump");
                    linear_velocity.y = player_props.jump_impulse;
                }
            }

            // Increment ground tick but cap at max value
            logical_controller.ground_tick = logical_controller.ground_tick.saturating_add(1);
        } else {
            // airborne
            info!("airborne");

            logical_controller.ground_tick = 0;
            wish_speed = f32::min(wish_speed, player_props.air_speed_cap);

            let mut add = accelerate(
                wish_direction,
                wish_speed,
                player_props.air_acceleration,
                linear_velocity.0,
                delta_time,
            );
            add.y = -player_props.gravity * delta_time;
            linear_velocity.0 += add;

            let air_speed = linear_velocity.xz().length();
            if air_speed > player_props.max_air_speed {
                let ratio = player_props.max_air_speed / air_speed;
                linear_velocity.x *= ratio;
                linear_velocity.z *= ratio;
            }
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

/// Return a collider that is scaled laterally (XZ plane) but not vertically (Y axis).
fn scaled_collider_laterally(collider: &Collider, scale: f32) -> Collider {
    if let Some(cylinder) = collider.shape().as_cylinder() {
        let new_cylinder = Collider::cylinder(cylinder.radius * scale, cylinder.half_height * 2.0);
        new_cylinder
    } else {
        panic!("Controller must use a cylinder  collider")
    }
}


fn accelerate(
    wish_direction: Vec3,
    wish_speed: f32,
    acceleration: f32,
    velocity: Vec3,
    dt: f32,
) -> Vec3 {
    let velocity_projection = Vec3::dot(velocity, wish_direction);
    let add_speed = wish_speed - velocity_projection;
    if add_speed <= 0.0 {
        return Vec3::ZERO;
    }

    let acceleration_speed = f32::min(acceleration * wish_speed * dt, add_speed);
    wish_direction * acceleration_speed
}