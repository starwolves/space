use std::time::Duration;

use bevy::prelude::{App, CoreSchedule, CoreSet, IntoSystemAppConfig, IntoSystemConfig, Plugin};
use bevy::time::common_conditions::on_fixed_timer;
use networking::messaging::{register_reliable_message, MessageSender};
use resources::is_server::is_server;
use resources::labels::{ActionsLabels, PostUpdateLabels, StartupLabels};

use crate::entity_data::{world_mode_update, InterpolationSet, RawSpawnEvent};
use crate::entity_types::{finalize_register_entity_types, EntityTypeLabel, EntityTypes};
use crate::examine::{
    examine_entity, examine_entity_health, finalize_entity_examine_input, finalize_examine_entity,
    incoming_messages, ExamineEntityMessages, InputExamineEntity,
};
use crate::finalize_entity_updates::finalize_entity_updates;
use crate::init::load_ron_entities;
use crate::net::{EntityClientMessage, EntityServerMessage};
use crate::spawn::{ClientEntityServerEntity, PawnEntityId};
use crate::spawning_events::{despawn_entity, DespawnClientEntity, SpawnClientEntity};
use crate::visible_checker::visible_checker;

use super::entity_data::broadcast_position_updates;

pub struct EntityPlugin;
impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.add_system(
                broadcast_position_updates
                    .in_schedule(CoreSchedule::FixedUpdate)
                    .in_set(InterpolationSet::Main)
                    .run_if(on_fixed_timer(Duration::from_secs_f32(1. / 2.))),
            )
            .add_system(
                world_mode_update
                    .in_set(PostUpdateLabels::EntityUpdate)
                    .in_base_set(CoreSet::PostUpdate),
            )
            .add_system(
                visible_checker
                    .in_set(PostUpdateLabels::VisibleChecker)
                    .after(PostUpdateLabels::SendEntityUpdates)
                    .in_base_set(CoreSet::PostUpdate),
            )
            .add_system(
                despawn_entity
                    .in_base_set(CoreSet::PostUpdate)
                    .after(PostUpdateLabels::VisibleChecker),
            )
            .add_system(
                finalize_examine_entity
                    .before(PostUpdateLabels::EntityUpdate)
                    .in_base_set(CoreSet::PostUpdate),
            )
            .add_system(examine_entity_health.after(ActionsLabels::Action))
            .init_resource::<ExamineEntityMessages>()
            .add_system(finalize_entity_examine_input.in_base_set(CoreSet::PreUpdate))
            .add_system(examine_entity.after(ActionsLabels::Action))
            .add_system(incoming_messages.in_base_set(CoreSet::PreUpdate))
            .add_event::<InputExamineEntity>()
            .add_system(
                finalize_entity_updates
                    .in_base_set(CoreSet::PostUpdate)
                    .after(PostUpdateLabels::EntityUpdate)
                    .in_set(PostUpdateLabels::SendEntityUpdates),
            )
            .add_event::<DespawnClientEntity>()
            .add_event::<SpawnClientEntity>();
        } else {
            app.init_resource::<PawnEntityId>()
                .init_resource::<ClientEntityServerEntity>();
        }
        app.add_event::<RawSpawnEvent>()
            .init_resource::<EntityTypes>()
            .add_startup_system(finalize_register_entity_types.after(EntityTypeLabel::Register))
            .add_startup_system(
                load_ron_entities
                    .after(StartupLabels::BuildGridmap)
                    .in_set(StartupLabels::InitEntities),
            );
        register_reliable_message::<EntityServerMessage>(app, MessageSender::Server);
        register_reliable_message::<EntityClientMessage>(app, MessageSender::Client);
    }
}
