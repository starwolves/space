use api::data::{CombatLabels, StartupLabels, SummoningLabels};
use bevy::prelude::{App, ParallelSystemDescriptorCoercion, Plugin, ResMut};
use combat::sfx::health_combat_hit_result_sfx;
use entity::{
    entity_data::initialize_entity_data,
    meta::{EntityDataProperties, EntityDataResource},
    spawn::{summon_base_entity, SpawnEvent},
};
use rigid_body::spawn::summon_rigid_body;

use crate::computer::Computer;

use super::{
    computer::computer_added,
    spawn::{
        default_summon_computer, summon_computer, summon_raw_computer, ComputerSummoner,
        BRIDGE_COMPUTER_ENTITY_NAME,
    },
};

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
            )
            .add_system(
                health_combat_hit_result_sfx::<Computer>.after(CombatLabels::FinalizeApplyDamage),
            );
    }
}

fn content_initialization(mut entity_data: ResMut<EntityDataResource>) {
    let entity_properties = EntityDataProperties {
        name: BRIDGE_COMPUTER_ENTITY_NAME.to_string(),
        id: entity_data.get_id_inc(),
        ..Default::default()
    };
    initialize_entity_data(&mut entity_data, entity_properties);
}
