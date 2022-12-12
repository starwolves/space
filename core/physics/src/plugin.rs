use std::env;

use bevy::prelude::{App, Plugin};

use crate::{out_of_bounds_teleportation::out_of_bounds_tp, rigidbody_link_transform::rigidbody_link_transform};

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        if env::var("CARGO_MANIFEST_DIR").unwrap().ends_with("server") {
            app.add_system(out_of_bounds_tp)
                .add_system(rigidbody_link_transform);
        }
    }
}
