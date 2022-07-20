use std::{fs, path::Path};

use bevy::prelude::ResMut;
use api::world_environment::{WorldEnvironment, WorldEnvironmentRaw};

pub fn startup_environment(mut map_environment: ResMut<WorldEnvironment>) {
    let environment_json_location = Path::new("data")
        .join("maps")
        .join("bullseye")
        .join("environment.json");
    let current_map_environment_raw_json: String = fs::read_to_string(environment_json_location)
        .expect("main.rs main() Error reading map environment.json file from drive.");
    let current_map_raw_environment: WorldEnvironmentRaw =
        serde_json::from_str(&current_map_environment_raw_json)
            .expect("main.rs main() Error parsing map environment.json String.");
    let current_map_environment: WorldEnvironment =
        WorldEnvironment::new(current_map_raw_environment);

    current_map_environment.adjust(&mut map_environment);
}
