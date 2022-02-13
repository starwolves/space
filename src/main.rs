use bevy::{core::DefaultTaskPoolOptions, prelude::App};
use space::SpacePlugin;

pub mod space;
pub mod plugins;


fn main() {
    App::new()
        //.insert_resource(ReportExecutionOrderAmbiguities)
        .insert_resource(DefaultTaskPoolOptions::with_num_threads(1))
        .add_plugin(SpacePlugin)
        .run();
}
