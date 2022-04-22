use bevy_app::{App, Plugin};

use self::systems::computer_added;

pub mod components;
pub mod spawn;
pub mod systems;

pub struct ComputersPlugin;

impl Plugin for ComputersPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(computer_added);
    }
}
