use bevy::prelude::{App, IntoSystemDescriptor, Plugin, SystemSet};
use bevy::time::FixedTimestep;
use networking::messaging::{init_reliable_message, MessageSender};
use resources::is_server::is_server;
use resources::labels::{ActionsLabels, PostUpdateLabels, StartupLabels};

use crate::entity_data::{world_mode_update, RawSpawnEvent, INTERPOLATION_LABEL1};
use crate::entity_types::{finalize_register_entity_types, EntityTypeLabel, EntityTypes};
use crate::examine::{
    examine_entity, examine_entity_health, finalize_entity_examine_input, finalize_examine_entity,
    incoming_messages, ExamineEntityMessages, InputExamineEntity,
};
use crate::finalize_entity_updates::finalize_entity_updates;
use crate::init::load_ron_entities;
use crate::loading::load_entities;
use crate::meta::EntityDataResource;
use crate::net::{EntityClientMessage, EntityServerMessage};
use crate::spawn::DefaultSpawnEvent;
use crate::spawning_events::{
    despawn_entity, spawn_entity_for_client, DespawnClientEntity, SpawnClientEntity,
};
use crate::visible_checker::visible_checker;

use super::entity_data::broadcast_position_updates;
use bevy::app::CoreStage::PostUpdate;
use bevy::app::CoreStage::PreUpdate;

pub struct EntityPlugin;
impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.add_event::<DefaultSpawnEvent>()
                .add_system_set(
                    SystemSet::new()
                        .with_run_criteria(
                            FixedTimestep::step(1. / 2.).with_label(INTERPOLATION_LABEL1),
                        )
                        .with_system(broadcast_position_updates),
                )
                .add_system_set_to_stage(
                    PostUpdate,
                    SystemSet::new()
                        .label(PostUpdateLabels::EntityUpdate)
                        .with_system(world_mode_update),
                )
                .add_system_to_stage(
                    PostUpdate,
                    visible_checker
                        .after(PostUpdateLabels::SendEntityUpdates)
                        .label(PostUpdateLabels::VisibleChecker),
                )
                .add_system_to_stage(
                    PostUpdate,
                    despawn_entity.after(PostUpdateLabels::VisibleChecker),
                )
                .add_system_to_stage(
                    PostUpdate,
                    finalize_examine_entity.before(PostUpdateLabels::EntityUpdate),
                )
                .add_system(examine_entity_health.after(ActionsLabels::Action))
                .init_resource::<ExamineEntityMessages>()
                .add_system_to_stage(PreUpdate, finalize_entity_examine_input)
                .add_system(examine_entity.after(ActionsLabels::Action))
                .add_system_to_stage(PreUpdate, incoming_messages)
                .add_event::<InputExamineEntity>()
                .add_system_to_stage(
                    PostUpdate,
                    finalize_entity_updates
                        .after(PostUpdateLabels::EntityUpdate)
                        .label(PostUpdateLabels::SendEntityUpdates),
                )
                .add_system(spawn_entity_for_client)
                .add_event::<DespawnClientEntity>()
                .add_event::<SpawnClientEntity>();
        } else {
            app.add_system(load_entities);
        }
        app.add_event::<RawSpawnEvent>()
            .init_resource::<EntityDataResource>()
            .init_resource::<EntityTypes>()
            .add_startup_system(finalize_register_entity_types.after(EntityTypeLabel::Register))
            .add_startup_system(
                load_ron_entities
                    .after(StartupLabels::BuildGridmap)
                    .label(StartupLabels::InitEntities),
            );
        init_reliable_message::<EntityServerMessage>(app, MessageSender::Server);
        init_reliable_message::<EntityClientMessage>(app, MessageSender::Client);
    }
}
