use bevy::prelude::{App, Plugin};

use super::{
    out_of_bounds_check::out_of_bounds_check, rigidbody_link_transform::rigidbody_link_transform,
};

pub struct RigidBodyPlugin;
impl Plugin for RigidBodyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(out_of_bounds_check)
            .add_system(rigidbody_link_transform);
    }
}
