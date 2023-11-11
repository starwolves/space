use bevy::prelude::{App, FixedUpdate, IntoSystemConfigs, Plugin, Update};
use bevy_xpbd_3d::{prelude::PhysicsPlugins, resources::SubstepCount};
use networking::{messaging::MessagingSet, stamp::step_tickrate_stamp};
use resources::{core::TickRate, modes::is_server_mode, sets::MainSet};

use crate::{
    cache::{cache_data, PhysicsCache},
    correction_mode::{CorrectionResults, StartCorrection},
    entity::{
        client_interpolate_link_transform, client_mirror_link_target_transform, remove_links,
        server_mirror_link_transform, ResetLerp, RigidBodies,
    },
    mirror_physics_transform::rigidbody_link_transform,
    sync::{sync_loop, FastForwarding, SyncPause},
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
        } else {
            app.add_systems(
                FixedUpdate,
                (
                    client_mirror_link_target_transform.in_set(MainSet::PreUpdate),
                    cache_data.in_set(MainSet::PostUpdate),
                ),
            )
            .add_systems(
                Update,
                client_interpolate_link_transform
                    .after(client_mirror_link_target_transform)
                    .in_set(MainSet::PreUpdate),
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
            .add_event::<StartCorrection>()
            .add_event::<CorrectionResults>();
        }
        app.add_plugins(PhysicsPlugins::new(FixedUpdate))
            .init_resource::<RigidBodies>()
            .add_systems(FixedUpdate, remove_links.in_set(MainSet::PostUpdate))
            .insert_resource(SubstepCount(TickRate::default().physics_substep.into()))
            .init_resource::<PhysicsCache>();
    }
}
