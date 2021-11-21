use bevy::{core::DefaultTaskPoolOptions, prelude::App};
use space_core::SpaceCore;
pub mod space_core;
pub mod plugins;


fn main() {

    // Amount of threads the Rapier physics engine will use when adding the "parallel" feature to bevy_rapier3d in cargo.toml.
    //std::env::set_var("RAYON_NUM_THREADS ", 2);

    App::build()
        .insert_resource(DefaultTaskPoolOptions::with_num_threads(2))
        .add_plugin(SpaceCore)
        .run();
}
