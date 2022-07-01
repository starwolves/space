use bevy_app::{App, Plugin};
use bevy_ecs::{schedule::ParallelSystemDescriptorCoercion, system::ResMut};

use crate::core::{
    entity::{
        functions::initialize_entity_data::initialize_entity_data,
        resources::{EntityDataProperties, EntityDataResource},
        spawn::{summon_base_entity, SpawnEvent},
    },
    rigid_body::spawn::summon_rigid_body,
    StartupLabels, SummoningLabels,
};

use self::{
    spawn::{
        default_summon_computer, summon_computer, summon_raw_computer, ComputerSummoner,
        BRIDGE_COMPUTER_ENTITY_NAME,
    },
    systems::computer_added,
};

pub mod components;
pub mod spawn;
pub mod systems;

pub struct ComputersPlugin;

impl Plugin for ComputersPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(computer_added)
            .add_event::<SpawnEvent<ComputerSummoner>>()
            .add_startup_system(content_initialization.before(StartupLabels::BuildGridmap))
            .add_system(summon_computer::<ComputerSummoner>.after(SummoningLabels::TriggerSummon))
            .add_system(
                (summon_base_entity::<ComputerSummoner>).after(SummoningLabels::TriggerSummon),
            )
            .add_system(
                (summon_rigid_body::<ComputerSummoner>).after(SummoningLabels::TriggerSummon),
            )
            .add_system((summon_raw_computer).after(SummoningLabels::TriggerSummon))
            .add_system(
                (default_summon_computer)
                    .label(SummoningLabels::DefaultSummon)
                    .after(SummoningLabels::NormalSummon),
            );
    }
}

pub fn content_initialization(mut entity_data: ResMut<EntityDataResource>) {
    let entity_properties = EntityDataProperties {
        name: BRIDGE_COMPUTER_ENTITY_NAME.to_string(),
        id: entity_data.get_id_inc(),
        ..Default::default()
    };
    initialize_entity_data(&mut entity_data, entity_properties);
}
