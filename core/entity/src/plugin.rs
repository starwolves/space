use std::time::Duration;

use bevy::prelude::{App, FixedUpdate, IntoSystemConfigs, Plugin, Startup};
use bevy::time::common_conditions::on_fixed_timer;
use networking::messaging::{register_reliable_message, MessageSender};
use resources::is_server::is_server;
use resources::sets::{ActionsSet, MainSet, PostUpdateSet, StartupSet};

use crate::entity_data::{world_mode_update, InterpolationSet, RawSpawnEvent};
use crate::entity_types::{finalize_register_entity_types, EntityTypeLabel, EntityTypes};
use crate::examine::{
    examine_entity, examine_entity_health, finalize_entity_examine_input, finalize_examine_entity,
    incoming_messages, ExamineEntityMessages, InputExamineEntity,
};
use crate::finalize_entity_updates::finalize_entity_updates;
use crate::init::load_ron_entities;
use crate::net::{EntityClientMessage, EntityServerMessage};
use crate::spawn::{ClientEntityServerEntity, PawnId};
use crate::spawning_events::{despawn_entity, DespawnClientEntity, SpawnClientEntity};
use crate::visible_checker::visible_checker;

use super::entity_data::broadcast_position_updates;

pub struct EntityPlugin;
impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.add_systems(
                FixedUpdate,
                broadcast_position_updates
                    .in_set(InterpolationSet::Main)
                    .run_if(on_fixed_timer(Duration::from_secs_f32(1. / 2.)))
                    .in_set(MainSet::Update),
            )
            .add_systems(
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
            .add_systems(
                FixedUpdate,
                finalize_entity_updates
                    .after(PostUpdateSet::EntityUpdate)
                    .in_set(PostUpdateSet::SendEntityUpdates)
                    .in_set(MainSet::PostUpdate),
            )
            .add_event::<DespawnClientEntity>()
            .add_event::<SpawnClientEntity>();
        } else {
            app.init_resource::<ClientEntityServerEntity>();
        }
        app.add_event::<RawSpawnEvent>()
            .init_resource::<PawnId>()
            .init_resource::<EntityTypes>()
            .add_systems(
                Startup,
                (finalize_register_entity_types.after(EntityTypeLabel::Register),),
            )
            .add_systems(
                FixedUpdate,
                load_ron_entities
                    .after(StartupSet::BuildGridmap)
                    .in_set(MainSet::PreUpdate)
                    .in_set(StartupSet::InitEntities),
            );
        register_reliable_message::<EntityServerMessage>(app, MessageSender::Server);
        register_reliable_message::<EntityClientMessage>(app, MessageSender::Client);
    }
}
