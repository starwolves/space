use bevy::prelude::{
    resource_exists, App, Condition, FixedUpdate, IntoSystemConfigs, Plugin, Startup,
};
use bevy_renet::renet::{RenetClient, RenetServer};
use networking::messaging::{register_reliable_message, MessageSender};
use resources::modes::is_server_mode;
use resources::sets::{ActionsSet, BuildingSet, MainSet, PostUpdateSet, StartupSet};

use crate::despawn::{despawn_entities, DespawnEntity};
use crate::entity_data::{world_mode_update, RawSpawnEvent};
use crate::entity_types::{finalize_register_entity_types, EntityTypeLabel, EntityTypes};
use crate::examine::{
    examine_entity, examine_entity_health, finalize_entity_examine_input, finalize_examine_entity,
    incoming_messages, ExamineEntityMessages, InputExamineEntity,
};
use crate::init::load_ron_entities;
use crate::loading::link_peer;
use crate::net::{EntityClientMessage, EntityServerMessage};
use crate::spawn::{ClientEntityServerEntity, PawnId, PeerPawns};
use crate::spawning_events::{despawn_entity, DespawnClientEntity, SpawnClientEntity};
use crate::visible_checker::visible_checker;

pub struct EntityPlugin;
impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        if is_server_mode(app) {
            app.add_systems(
                FixedUpdate,
                (
                    despawn_entity.after(PostUpdateSet::VisibleChecker),
                    finalize_examine_entity.before(PostUpdateSet::EntityUpdate),
                    visible_checker
                        .in_set(PostUpdateSet::VisibleChecker)
                        .after(PostUpdateSet::SendEntityUpdates),
                    world_mode_update.in_set(PostUpdateSet::EntityUpdate),
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
            /*.add_systems(
                FixedUpdate,
                finalize_entity_updates
                    .after(PostUpdateSet::EntityUpdate)
                    .in_set(PostUpdateSet::SendEntityUpdates)
                    .in_set(MainSet::PostUpdate),
            )*/
            .add_event::<DespawnClientEntity>()
            .add_event::<SpawnClientEntity>();
        } else {
            app.init_resource::<ClientEntityServerEntity>()
                .init_resource::<PeerPawns>()
                .add_systems(
                    FixedUpdate,
                    link_peer
                        .in_set(MainSet::Update)
                        .after(BuildingSet::TriggerBuild),
                );
        }
        app.add_event::<DespawnEntity>()
            .add_event::<RawSpawnEvent>()
            .init_resource::<PawnId>()
            .init_resource::<EntityTypes>()
            .add_systems(
                Startup,
                (finalize_register_entity_types.after(EntityTypeLabel::Register),),
            )
            .add_systems(
                FixedUpdate,
                (
                    load_ron_entities
                        .after(StartupSet::BuildGridmap)
                        .in_set(MainSet::PreUpdate)
                        .in_set(StartupSet::InitEntities)
                        .in_set(BuildingSet::RawTriggerBuild)
                        .run_if(
                            resource_exists::<RenetClient>()
                                .or_else(resource_exists::<RenetServer>()),
                        ),
                    despawn_entities.in_set(MainSet::PostUpdate),
                ),
            );
        register_reliable_message::<EntityServerMessage>(app, MessageSender::Server, true);
        register_reliable_message::<EntityClientMessage>(app, MessageSender::Client, true);
    }
}
