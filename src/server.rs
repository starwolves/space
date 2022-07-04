use std::time::Duration;

use bevy::{
    app::{RunMode, ScheduleRunnerSettings},
    core::DefaultTaskPoolOptions,
    prelude::App,
};

use crate::core::space_plugin::plugin::SpacePlugin;

pub fn server() {
    App::new()
        //.insert_resource(ReportExecutionOrderAmbiguities)
        .insert_resource(DefaultTaskPoolOptions::with_num_threads(2))
        .insert_resource(ScheduleRunnerSettings {
            run_mode: RunMode::Loop {
                wait: Some(Duration::from_secs_f64(1. / 64.)),
            },
        })
        .add_plugin(SpacePlugin)
        .run();
}
