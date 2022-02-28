use bevy_core::DefaultTaskPoolOptions;
use bevy_internal::prelude::App;
use space::SpacePlugin;

pub mod plugins;
pub mod space;

fn main() {
    App::new()
        //.insert_resource(ReportExecutionOrderAmbiguities)
        .insert_resource(DefaultTaskPoolOptions::with_num_threads(2))
        .add_plugin(SpacePlugin)
        .run();
}
