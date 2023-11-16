use bevy::prelude::{App, FixedUpdate, IntoSystemConfigs, Plugin, Update};
use bevy_xpbd_3d::{prelude::PhysicsPlugins, resources::SubstepCount};
use networking::{messaging::MessagingSet, stamp::step_tickrate_stamp};
use resources::{
    core::TickRate,
    correction::CorrectionSet,
    modes::{is_correction_mode, is_server_mode},
    sets::MainSet,
};

use crate::{
    cache::{cache_data, PhysicsCache, PhysicsSet},
    correction_mode::CorrectionResults,
    entity::{
        client_interpolate_link_transform, client_mirror_link_target_transform, remove_links,
        server_mirror_link_transform, ResetLerp, RigidBodies,
    },
    mirror_physics_transform::rigidbody_link_transform,
    sync::{
        sync_entities, sync_loop, sync_physics_data, CorrectionServerRigidBodyLink, FastForwarding,
        SyncPause,
    },
};

pub struct PhysicsPlugin;
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        if is_server_mode(app) {
            app.add_systems(
                FixedUpdate,
                rigidbody_link_transform.in_set(MainSet::Update),
            )
            .add_systems(
                FixedUpdate,
                server_mirror_link_transform.in_set(MainSet::PreUpdate),
            );
            if is_correction_mode(app) {
                app.add_systems(
                    FixedUpdate,
                    (
                        sync_entities
                            .after(CorrectionSet::Start)
                            .in_set(MainSet::Update),
                        sync_physics_data
                            .in_set(MainSet::PreUpdate)
                            .in_set(CorrectionSet::SyncData),
                    ),
                )
                .init_resource::<CorrectionServerRigidBodyLink>();
            }
        } else {
            app.add_systems(
                FixedUpdate,
                (
                    client_mirror_link_target_transform
                        .after(MainSet::PostUpdate)
                        .after(PhysicsSet::Correct),
                    cache_data
                        .after(MainSet::PostUpdate)
                        .in_set(PhysicsSet::Cache),
                ),
            )
            .add_systems(
                Update,
                client_interpolate_link_transform
                    .after(client_mirror_link_target_transform)
                    .after(MainSet::PostUpdate),
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
    }
}
