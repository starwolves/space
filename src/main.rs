use bevy::{app, core::CorePlugin, log::LogPlugin, prelude::*, reflect::ReflectPlugin};
use bevy_rapier3d::{na::Point3, physics::RapierPhysicsPlugin};
use bevy_rapier3d::rapier::na::Vector;

use bevy_rapier3d::rapier::dynamics::RigidBodyBuilder;
use bevy_rapier3d::rapier::dynamics::RigidBody;

use bevy_rapier3d::rapier::geometry::ColliderBuilder;



struct PhysicsDynamicRigidBody {
    positon: Vec3,
    velocity: Vec3
}

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_startup_system(launch_server.system())
        .add_system(interpolate_entity_system.system())
        .run();
}

fn launch_server(commands: &mut Commands) {
    commands.spawn((
        RigidBodyBuilder::new_static().translation(0., 0., 0.),
        ColliderBuilder::cuboid(64., 0., 64.),
    ));


    commands.spawn((
        RigidBodyBuilder::new_dynamic().translation(0., 10., 0.),
        ColliderBuilder::cuboid(0.5, 0.5, 0.5),
        PhysicsDynamicRigidBody {
            positon: Vec3::new(0.,0.,0.),
            velocity: Vec3::new(0.,0.,0.)
        }
    ));

}

fn interpolate_entity_system(mut query: Query<(&mut RigidBody)>) {

    for dynamicRigidBody in query.iter_mut() {

        info!("Falling cube is at position {} !", dynamicRigidBody.position());

    }

}