use bevy::prelude::App;
use space_core::SpaceCore;
pub mod space_core;
pub mod plugins;


fn main() {
    App::build()
        .add_plugin(SpaceCore)
        .run();
}
