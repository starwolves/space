pub mod core;
pub mod entities;

use std::time::Duration;

use self::core::SpacePlugin;

use bevy_app::{App, RunMode, ScheduleRunnerSettings};
use bevy_core::DefaultTaskPoolOptions;
fn main() {
    App::new()
        //.insert_resource(ReportExecutionOrderAmbiguities)
        .insert_resource(DefaultTaskPoolOptions::with_num_threads(2))
        .insert_resource(ScheduleRunnerSettings {
            run_mode: RunMode::Loop {
                wait: Some(Duration::from_secs_f64(1. / 60.)),
            },
        })
        .add_plugin(SpacePlugin)
        .run();
}
