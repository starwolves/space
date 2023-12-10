use std::time::Duration;

use bevy::{
    prelude::{App, FixedUpdate, IntoSystemConfigs, Plugin, Update},
    time::common_conditions::on_timer,
};
use bevy_xpbd_3d::{prelude::PhysicsPlugins, resources::SubstepCount};
use networking::{
    messaging::{register_reliable_message, MessageSender, MessagingSet},
    stamp::step_tickrate_stamp,
};
use resources::{
    core::TickRate,
    correction::CorrectionSet,
    modes::{is_correction_mode, is_server, is_server_mode},
    physics::PhysicsSet,
    sets::MainSet,
};

use crate::{
    cache::{cache_data, PhysicsCache},
    correction_mode::CorrectionResults,
    entity::{
        client_interpolate_link_transform, client_mirror_link_target_transform, remove_links,
        server_mirror_link_transform, ResetLerp, RigidBodies,
    },
    mirror_physics_transform::rigidbody_link_transform,
    net::PhysicsServerMessage,
    sync::{
        desync_check_correction, send_desync_check, sync_correction_world_entities, sync_loop,
        sync_physics_data, CorrectionServerRigidBodyLink, FastForwarding, SpawningSimulation,
        SpawningSimulationRigidBody, SyncPause,
    },
};

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        if is_server_mode(app) {
            app.add_systems(
                FixedUpdate,
                rigidbody_link_transform.in_set(MainSet::Update),
            );
            if !is_correction_mode(app) {
                app.add_systems(
                    FixedUpdate,
                    server_mirror_link_transform.in_set(MainSet::PreUpdate),
                );
            }
            if is_correction_mode(app) {
                app.add_systems(
                    FixedUpdate,
                    (
                        sync_correction_world_entities
                            .after(CorrectionSet::Start)
                            .in_set(MainSet::Update)
                            .before(SpawningSimulation::Spawn),
                        sync_physics_data.in_set(MainSet::PostPhysics),
                    ),
                )
                .init_resource::<CorrectionServerRigidBodyLink>()
                .add_event::<SpawningSimulationRigidBody>();
            }
            if is_server() {
                app.add_systems(
                    FixedUpdate,
                    (send_desync_check
                        .in_set(MainSet::Update)
                        .run_if(on_timer(Duration::from_secs_f32(0.25))),),
                );
            }
        } else {
            app.add_systems(
                FixedUpdate,
                (
                    client_mirror_link_target_transform
                        .in_set(MainSet::PostPhysics)
                        .after(PhysicsSet::Correct),
                    // Cache twice.
                    cache_data
                        .after(MainSet::PostUpdate)
                        .in_set(PhysicsSet::Cache),
                    cache_data
                        .in_set(MainSet::PostPhysics)
                        .after(PhysicsSet::Correct),
                    desync_check_correction
                        .in_set(MainSet::Update)
                        .in_set(CorrectionSet::Start),
                ),
            )
            .add_systems(
                Update,
                client_interpolate_link_transform
                    .after(client_mirror_link_target_transform)
                    .after(MainSet::PostPhysics),
            )
            .add_event::<ResetLerp>()
            .init_resource::<SyncPause>()
            .add_systems(
                FixedUpdate,
                sync_loop
                    .before(step_tickrate_stamp)
                    .after(MessagingSet::DeserializeIncoming)
                    .in_set(MainSet::PreUpdate),
            )
            .init_resource::<FastForwarding>()
            .add_event::<CorrectionResults>();
        }
        app.add_plugins(PhysicsPlugins::new(FixedUpdate))
            .init_resource::<RigidBodies>()
            .add_systems(FixedUpdate, remove_links.in_set(MainSet::PostUpdate))
            .insert_resource(SubstepCount(TickRate::default().physics_substep.into()))
            .init_resource::<PhysicsCache>();
        register_reliable_message::<PhysicsServerMessage>(app, MessageSender::Server, false);
    }
}
