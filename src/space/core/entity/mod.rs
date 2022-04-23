use bevy_app::CoreStage::PostUpdate;
use bevy_app::{App, Plugin};
use bevy_core::FixedTimestep;
use bevy_ecs::schedule::{ParallelSystemDescriptorCoercion, SystemSet};
use bevy_ecs::system::Res;
use bevy_log::info;

use crate::space::{PostUpdateLabels, StartupLabels};

use self::systems::entity_console_commands::entity_console_commands;
use self::{
    events::{NetLoadEntity, NetSendEntityUpdates, NetShowcase, NetUnloadEntity},
    resources::EntityDataResource,
    systems::{
        broadcast_position_updates::{broadcast_position_updates, INTERPOLATION_LABEL1},
        send_entity_updates::send_entity_updates,
    },
};

pub mod components;
pub mod events;
pub mod functions;
pub mod resources;
pub mod systems;

pub fn startup_entities(entity_data: Res<EntityDataResource>) {
    info!("Loaded {} different entity types.", entity_data.data.len());
}

pub struct EntityPlugin;
impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EntityDataResource>()
            .add_system_to_stage(
                PostUpdate,
                send_entity_updates
                    .after(PostUpdateLabels::EntityUpdate)
                    .label(PostUpdateLabels::SendEntityUpdates),
            )
            .add_event::<NetShowcase>()
            .add_event::<NetSendEntityUpdates>()
            .add_event::<NetUnloadEntity>()
            .add_event::<NetLoadEntity>()
            .add_system_set(
                SystemSet::new()
                    .with_run_criteria(
                        FixedTimestep::step(1. / 2.).with_label(INTERPOLATION_LABEL1),
                    )
                    .with_system(broadcast_position_updates),
            )
            .add_startup_system(
                startup_entities
                    .before(StartupLabels::BuildGridmap)
                    .label(StartupLabels::InitEntities),
            )
            .add_system(entity_console_commands);
    }
}
