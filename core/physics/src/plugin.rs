use bevy::{
    ecs::schedule::common_conditions::resource_exists,
    prelude::{App, FixedUpdate, IntoSystemConfigs, Plugin, Update},
};
use bevy_renet::renet::RenetClient;
use bevy_xpbd_3d::{prelude::PhysicsPlugins, resources::SubstepCount};
use networking::messaging::{register_unreliable_message, MessageSender, MessagingSet};
use resources::{
    core::TickRate,
    correction::CorrectionSet,
    modes::{is_correction_mode, is_server_mode},
    physics::{PhysicsSet, PriorityPhysicsCache},
    sets::MainSet,
};

use crate::{
    cache::{
        apply_newly_spawned_data, cache_data, cache_data_second, clear_priority_cache,
        sync_entities, PhysicsCache, SyncEntitiesPhysics,
    },
    correction_mode::CorrectionResults,
    entity::{
        client_interpolate_link_transform, client_mirror_link_target_transform,
        remove_rigidbody_links, server_mirror_link_transform, ResetLerp, RigidBodies,
    },
    mirror_physics_transform::rigidbody_link_transform,
    net::PhysicsUnreliableServerMessage,
    out_of_bounds::despawn_out_of_bounds,
    spawn::{clear_new, NewlySpawnedRigidbodies},
    sync::{
        client_apply_priority_cache, client_despawn_and_clean_cache,
        correction_server_apply_priority_cache, desync_check_correction, init_physics_data,
        send_desync_check, start_sync, sync_correction_world_entities, sync_loop,
        ClientStartedSyncing, CorrectionServerRigidBodyLink, FastForwarding, SimulationStorage,
        SpawningSimulation, SpawningSimulationRigidBody, SyncPause,
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
                    (
                        server_mirror_link_transform.in_set(MainSet::PreUpdate),
                        despawn_out_of_bounds.in_set(MainSet::Update),
                        send_desync_check.in_set(MainSet::Update),
                    ),
                );
            } else {
                app.add_systems(
                    FixedUpdate,
                    (
                        sync_correction_world_entities
                            .after(CorrectionSet::Start)
                            .in_set(MainSet::Update)
                            .before(SpawningSimulation::Spawn),
                        init_physics_data.in_set(MainSet::PostPhysics),
                        correction_server_apply_priority_cache.in_set(MainSet::PreUpdate),
                    ),
                )
                .init_resource::<CorrectionServerRigidBodyLink>()
                .add_event::<SpawningSimulationRigidBody>()
                .init_resource::<SimulationStorage>();
            }
        } else {
            app.add_systems(
                FixedUpdate,
                (
                    client_mirror_link_target_transform
                        .in_set(MainSet::PostPhysics)
                        .after(PhysicsSet::Correct),
                    cache_data.in_set(MainSet::PreUpdate),
                    cache_data_second
                        .in_set(MainSet::PostPhysics)
                        .before(PhysicsSet::CacheNewSpawns),
                    apply_newly_spawned_data
                        .in_set(MainSet::PostPhysics)
                        .in_set(PhysicsSet::CacheNewSpawns),
                    desync_check_correction
                        .run_if(resource_exists::<RenetClient>())
                        .in_set(MainSet::Update)
                        .in_set(CorrectionSet::Start)
                        .before(sync_entities),
                    sync_entities.in_set(MainSet::Update),
                    client_despawn_and_clean_cache.in_set(MainSet::Update),
                    client_apply_priority_cache
                        .in_set(MainSet::PreUpdate)
                        .before(cache_data),
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
                (
                    sync_loop
                        .after(MessagingSet::DeserializeIncoming)
                        .in_set(MainSet::PreUpdate),
                    start_sync
                        .in_set(MainSet::PreUpdate)
                        .after(MessagingSet::DeserializeIncoming)
                        .before(sync_loop),
                    clear_priority_cache
                        .in_set(MainSet::PostPhysics)
                        .after(PhysicsSet::Correct),
                ),
            )
            .init_resource::<FastForwarding>()
            .add_event::<CorrectionResults>()
            .init_resource::<ClientStartedSyncing>()
            .add_event::<SyncEntitiesPhysics>();
        }
        app.add_plugins(PhysicsPlugins::new(FixedUpdate))
            .init_resource::<RigidBodies>()
            .add_systems(
                FixedUpdate,
                remove_rigidbody_links.in_set(MainSet::PostUpdate),
            )
            .insert_resource(SubstepCount(TickRate::default().physics_substep.into()))
            .init_resource::<PhysicsCache>()
            .init_resource::<PriorityPhysicsCache>()
            .init_resource::<NewlySpawnedRigidbodies>()
            .add_systems(FixedUpdate, clear_new.in_set(MainSet::Update));

        register_unreliable_message::<PhysicsUnreliableServerMessage>(app, MessageSender::Server);
    }
}
