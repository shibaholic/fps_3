use avian3d::prelude::{LinearVelocity, Position, RigidBody};
use bevy::prelude::*;

use crate::player::component::Grounded;

pub struct RegisterPlugin;

impl Plugin for RegisterPlugin {
    fn build(&self, app: &mut App) {
        app
        .register_type::<RigidBody>()
        .register_type::<LinearVelocity>()
        .register_type::<Position>()
        .register_type::<Grounded>()
        ;
    }
}