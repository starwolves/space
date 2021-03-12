use bevy::{app, prelude::*};
use bevy_rapier3d::{na::Point3, physics::{RapierPhysicsPlugin}};
use bevy_rapier3d::rapier::na::Vector;

use bevy_rapier3d::rapier::dynamics::RigidBodyBuilder;
use bevy_rapier3d::rapier::dynamics::RigidBody;


use bevy_rapier3d::rapier::geometry::ColliderBuilder;

use bevy_rapier3d::physics::PhysicsInterpolationComponent;
use bevy_rapier3d::rapier::dynamics::RigidBodySet;
use bevy_rapier3d::physics::RigidBodyHandleComponent;

struct PhysicsDynamicRigidBodyComponent;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin)
        .add_startup_system(launch_server.system())
        .add_system(interpolate_entities_system.system())
        .run();
}

fn launch_server(commands: &mut Commands) {

    // Spawn floor and then a cube up high thatll fall down and hit it.

    commands.spawn((
        RigidBodyBuilder::new_static().translation(0., 0., 0.),
        ColliderBuilder::cuboid(64., 0., 64.),
    ));


    commands.spawn((
        RigidBodyBuilder::new_dynamic().translation(0., 100., 0.),
        ColliderBuilder::cuboid(0.5, 0.5, 0.5),
        PhysicsDynamicRigidBodyComponent {}
    ));

}

fn interpolate_entities_system(
    mut query: Query<
        (&RigidBodyHandleComponent, &PhysicsDynamicRigidBodyComponent)
    >,
    bodies: ResMut<RigidBodySet>
) {

    for (rigid_body_handle, traitDynamicRigidBody) in query.iter_mut() {

        

        if let Some(rigid_body) = bodies.get(rigid_body_handle.handle()) {
            
            info!("Falling cube is at position {} !", rigid_body.position());

        }

    }

}