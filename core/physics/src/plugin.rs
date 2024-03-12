use bevy::{
    ecs::{
        schedule::{common_conditions::resource_exists, ScheduleLabel, SystemSet},
        world::World,
    },
    log::warn,
    prelude::{App, IntoSystemConfigs, Plugin, Update as BevyUpdate},
};
use bevy_renet::renet::RenetClient;
use bevy_xpbd_3d::{prelude::PhysicsPlugins, resources::SubstepCount};
use entity::despawn::DespawnEntitySet;
use networking::{
    client::start_sync,
    messaging::{register_unreliable_message, MessageSender, MessagingSet},
};
use resources::{
    core::TickRate,
    correction::{SynchronousCorrection, SynchronousCorrectionOnGoing},
    modes::{is_correction_mode, is_server_mode},
    ordering::{Fin, PostUpdate, PreUpdate, Update},
    physics::PriorityPhysicsCache,
};

use crate::{
    cache::{
        apply_newly_spawned_data, cache_data_new_spawns, cache_data_prev_tick, clear_physics_cache,
        clear_priority_cache, sync_entities, PhysicsCache, SyncEntitiesPhysics,
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
        send_desync_check, sync_correction_world_entities, sync_loop, CorrectionEnabled,
        CorrectionServerRigidBodyLink, FastForwarding, SimulationStorage, SpawningSimulation,
        SpawningSimulationRigidBody, SyncPause,
    },
};

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        if is_server_mode(app) {
            app.add_systems(Update, rigidbody_link_transform);
            if !is_correction_mode(app) {
                app.add_systems(
                    Update,
                    (
                        despawn_out_of_bounds.before(DespawnEntitySet),
                        send_desync_check,
                    ),
                );
                app.add_systems(PreUpdate, (server_mirror_link_transform,));
            } else {
                app.add_systems(
                    PreUpdate,
                    (sync_correction_world_entities
                        .before(DespawnEntitySet)
                        .before(SpawningSimulation::Spawn),),
                )
                .add_systems(PreUpdate, (correction_server_apply_priority_cache,))
                .add_systems(PostUpdate, (init_physics_data.after(PhysicsStepSet),))
                .init_resource::<CorrectionServerRigidBodyLink>()
                .add_event::<SpawningSimulationRigidBody>()
                .init_resource::<SimulationStorage>()
                .init_resource::<CorrectionResults>();
            }
        } else {
            app.init_resource::<CorrectionResults>()
                .add_systems(Fin, client_apply_priority_cache)
                .add_systems(Fin, (client_mirror_link_target_transform,))
                .add_systems(
                    Update,
                    (
                        desync_check_correction
                            .run_if(resource_exists::<RenetClient>)
                            .before(sync_entities),
                        sync_entities,
                        client_despawn_and_clean_cache,
                        cache_data_new_spawns,
                        apply_newly_spawned_data.after(cache_data_new_spawns),
                    ),
                )
                .add_systems(BevyUpdate, client_interpolate_link_transform)
                .add_event::<ResetLerp>()
                .init_resource::<SyncPause>()
                .add_systems(
                    PreUpdate,
                    (
                        sync_loop
                            .before(start_sync)
                            .after(MessagingSet::DeserializeIncoming)
                            .run_if(resource_exists::<RenetClient>),
                        cache_data_prev_tick,
                    ),
                )
                .add_systems(Fin, (clear_priority_cache, clear_physics_cache))
                .init_resource::<FastForwarding>()
                .add_event::<SyncEntitiesPhysics>();
        }
        app.add_plugins(PhysicsPlugins::new(PhysicsStep))
            .init_resource::<RigidBodies>()
            .add_systems(Update, remove_rigidbody_links.in_set(DespawnEntitySet))
            .insert_resource(SubstepCount(TickRate::default().physics_substep.into()))
            .init_resource::<PhysicsCache>()
            .init_resource::<PriorityPhysicsCache>()
            .init_resource::<NewlySpawnedRigidbodies>()
            .add_systems(PostUpdate, clear_new)
            .add_systems(PostUpdate, physics_step.in_set(PhysicsStepSet))
            .init_resource::<CorrectionEnabled>();

        register_unreliable_message::<PhysicsUnreliableServerMessage>(app, MessageSender::Server);
    }
}
#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PhysicsStep;
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub struct PhysicsStepSet;

pub(crate) fn physics_step(world: &mut World) {
    let synchronous = world.resource::<SynchronousCorrection>().0;
    let test;
    if synchronous {
        test = !world
            .resource::<SynchronousCorrectionOnGoing>()
            .receive_ready();
    } else {
        test = !world.resource::<CorrectionEnabled>().0;
    }
    if test {
        match world.try_run_schedule(PhysicsStep) {
            Ok(_) => {}
            Err(rr) => {
                warn!("PhysicsStep: {}", rr);
            }
        }
    }
}
