use bevy::{core::DefaultTaskPoolOptions, prelude::App};
use space_core::SpaceCore;
pub mod space_core;
pub mod plugins;


fn main() {
    
    App::new()
        .insert_resource(DefaultTaskPoolOptions::with_num_threads(2))
        .add_plugin(SpaceCore)
        .run();
    
}
