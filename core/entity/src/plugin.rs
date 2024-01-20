use bevy::prelude::{
    resource_exists, App, Condition, FixedUpdate, IntoSystemConfigs, Plugin, Startup,
};
use bevy_renet::renet::{RenetClient, RenetServer};
use networking::messaging::{register_reliable_message, MessageSender, TypenamesSet};
use networking::server::EntityUpdatesSet;
use resources::modes::{is_correction_mode, is_server_mode};
use resources::sets::{ActionsSet, BuildingSet, MainSet, PostUpdateSet, StartupSet};

use crate::despawn::{client_despawn_entity, despawn_entities, DespawnEntity};
use crate::entity_data::{fire_queued_entity_updates, QueuedSpawnEntityUpdates, RawSpawnEvent};
use crate::entity_types::{
    clean_entity_type_cache, finalize_register_entity_types, EntityTypeCache, EntityTypeLabel,
    EntityTypes,
};
use crate::examine::{
    examine_entity, examine_entity_health, finalize_entity_examine_input, finalize_examine_entity,
    incoming_messages, ExamineEntityMessages, InputExamineEntity,
};
use crate::init::load_ron_entities;
use crate::loading::{link_peer, NewToBeCachedSpawnedEntities};
use crate::net::{EntityClientMessage, EntityServerMessage};
use crate::spawn::{PawnId, PeerPawns, ServerEntityClientEntity};
use crate::spawning_events::{
    construct_entity_updates, despawn_entity, DespawnClientEntity, SpawnClientEntity,
};
use crate::visible_checker::visible_checker;

pub struct EntityPlugin;
impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        if is_server_mode(app) {
            if !is_correction_mode(app) {
                app.add_systems(
                    FixedUpdate,
                    (
                        finalize_examine_entity,
                        visible_checker
                            .in_set(PostUpdateSet::VisibleChecker)
                            .in_set(EntityUpdatesSet::Write),
                    )
                        .in_set(MainSet::PostUpdate),
                )
                .add_systems(
                    FixedUpdate,
                    (
                        examine_entity.after(ActionsSet::Action),
                        examine_entity_health.after(ActionsSet::Action),
                    )
                        .in_set(MainSet::Update),
                )
                .init_resource::<ExamineEntityMessages>()
                .add_systems(
                    FixedUpdate,
                    (finalize_entity_examine_input, incoming_messages).in_set(MainSet::PreUpdate),
                )
                .add_event::<InputExamineEntity>()
                .add_systems(
                    FixedUpdate,
                    construct_entity_updates
                        .in_set(MainSet::PostUpdate)
                        .in_set(EntityUpdatesSet::Prepare),
                );
            }
            app.add_systems(
                FixedUpdate,
                (despawn_entity.after(PostUpdateSet::VisibleChecker),).in_set(MainSet::PostUpdate),
            )
            .add_event::<DespawnClientEntity>()
            .add_event::<SpawnClientEntity>();
        } else {
            app.init_resource::<ServerEntityClientEntity>()
                .init_resource::<PeerPawns>()
                .add_systems(
                    FixedUpdate,
                    (
                        link_peer
                            .in_set(MainSet::Update)
                            .after(BuildingSet::TriggerBuild),
                        fire_queued_entity_updates
                            .in_set(MainSet::PreUpdate)
                            .before(TypenamesSet::SendRawEvents),
                        client_despawn_entity.in_set(MainSet::Update),
                    ),
                )
                .init_resource::<QueuedSpawnEntityUpdates>()
                .init_resource::<NewToBeCachedSpawnedEntities>();
        }
        if !is_server_mode(app) {
            app.init_resource::<EntityTypeCache>()
                .add_systems(FixedUpdate, clean_entity_type_cache.in_set(MainSet::Update));
        }
        if is_correction_mode(app) {
            app.init_resource::<EntityTypeCache>();
        }

        if !is_correction_mode(app) {
            app.add_event::<RawSpawnEvent>()
                .init_resource::<PawnId>()
                .add_systems(
                    Startup,
                    (finalize_register_entity_types.after(EntityTypeLabel::Register),),
                )
                .add_systems(
                    FixedUpdate,
                    (load_ron_entities
                        .after(StartupSet::BuildGridmap)
                        .in_set(MainSet::PreUpdate)
                        .in_set(StartupSet::InitEntities)
                        .in_set(BuildingSet::RawTriggerBuild)
                        .run_if(
                            resource_exists::<RenetClient>()
                                .or_else(resource_exists::<RenetServer>()),
                        ),),
                );
        }

        app.add_event::<DespawnEntity>()
            .init_resource::<EntityTypes>()
            .add_systems(FixedUpdate, despawn_entities.in_set(MainSet::PostUpdate));
        register_reliable_message::<EntityServerMessage>(app, MessageSender::Server, true);
        register_reliable_message::<EntityClientMessage>(app, MessageSender::Client, true);
    }
}
