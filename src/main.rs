use bevy::prelude::*;
use bevy_rapier3d::physics::{RapierPhysicsPlugin};
use bevy_rapier3d::rapier::na::Vector;

fn main() {
    App::build()
    .add_plugin(RapierPhysicsPlugin)
    .add_startup_system(launch_server.system())
    .run();
}

fn launch_server(commands: &mut Commands) {



}
