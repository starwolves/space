use bevy::prelude::{App, IntoSystemConfigs, Plugin, Startup};
use networking::client::DeserializeSpawnUpdates;
use networking::messaging::{register_reliable_message, MessageSender, MessagingSet};
use networking::server::EntityUpdatesSet;
use resources::modes::{is_correction_mode, is_server_mode};
use resources::ordering::{ActionsSet, BuildingSet, PreUpdate, SensingSet, StartupSet, Update};

use crate::despawn::{client_despawn_entity, despawn_entities, DespawnEntity, DespawnEntitySet};
use crate::entity_data::{fire_load_entity_updates, QueuedSpawnEntityUpdates, RawSpawnEvent};
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
                    Update,
                    (
                        finalize_examine_entity,
                        visible_checker
                            .in_set(SensingSet::VisibleChecker)
                            .in_set(EntityUpdatesSet::Write),
                        examine_entity.after(ActionsSet::Action),
                        examine_entity_health.after(ActionsSet::Action),
                        construct_entity_updates.in_set(EntityUpdatesSet::Prepare),
                    ),
                )
                .init_resource::<ExamineEntityMessages>()
                .add_systems(
                    PreUpdate,
                    (finalize_entity_examine_input, incoming_messages)
                        .after(MessagingSet::DeserializeIncoming),
                )
                .add_event::<InputExamineEntity>();
            }
            app.add_systems(Update, (despawn_entity.after(SensingSet::VisibleChecker),))
                .add_event::<DespawnClientEntity>()
                .add_event::<SpawnClientEntity>();
        } else {
            app.init_resource::<ServerEntityClientEntity>()
                .init_resource::<PeerPawns>()
                .add_systems(Update, (client_despawn_entity.before(DespawnEntitySet),))
                .add_systems(
                    PreUpdate,
                    (
                        fire_load_entity_updates
                            .after(BuildingSet::TriggerBuild)
                            .before(DeserializeSpawnUpdates),
                        link_peer.after(BuildingSet::TriggerBuild),
                    ),
                )
                .init_resource::<QueuedSpawnEntityUpdates>()
                .init_resource::<NewToBeCachedSpawnedEntities>();
        }
        if !is_server_mode(app) {
            app.init_resource::<EntityTypeCache>()
                .add_systems(Update, clean_entity_type_cache.in_set(DespawnEntitySet));
        }
        if is_correction_mode(app) {
            app.init_resource::<EntityTypeCache>();
        }

        if !is_correction_mode(app) {
            app.add_event::<RawSpawnEvent>()
                .init_resource::<PawnId>()
                .add_systems(
                    Startup,
                    (
                        finalize_register_entity_types.after(EntityTypeLabel::Register),
                        load_ron_entities
                            .after(StartupSet::BuildGridmap)
                            .in_set(StartupSet::InitEntities)
                            .in_set(BuildingSet::RawTriggerBuild),
                    ),
                );
        }

        app.add_event::<DespawnEntity>()
            .init_resource::<EntityTypes>()
            .add_systems(Update, despawn_entities.in_set(DespawnEntitySet));
        register_reliable_message::<EntityServerMessage>(app, MessageSender::Server, true);
        register_reliable_message::<EntityClientMessage>(app, MessageSender::Client, true);
    }
}
