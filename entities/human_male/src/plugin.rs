use bevy::prelude::{App, ParallelSystemDescriptorCoercion, Plugin, ResMut};
use entity::{entity_data::initialize_entity_data, spawn::SpawnEvent};
use player_controller::humanoid::{
    default_human_dummy, summon_base_human_male, summon_human_male, HumanMaleSummoner,
};

use api::data::{
    CombatLabels, EntityDataProperties, EntityDataResource, StartupLabels, SummoningLabels,
    HUMAN_DUMMY_ENTITY_NAME, HUMAN_MALE_ENTITY_NAME,
};
use rigid_body::spawn::summon_rigid_body;

use crate::hands_attack_handler::hands_attack_handler;

pub struct HumanMalePlugin;

impl Plugin for HumanMalePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(content_initialization.before(StartupLabels::InitEntities))
            .add_system(
                summon_human_male::<HumanMaleSummoner>
                    .before(SummoningLabels::TriggerSummon)
                    .label(SummoningLabels::NormalSummon),
            )
            .add_system(
                (summon_base_human_male::<HumanMaleSummoner>).after(SummoningLabels::TriggerSummon),
            )
            .add_event::<SpawnEvent<HumanMaleSummoner>>()
            .add_system(
                (summon_rigid_body::<HumanMaleSummoner>).after(SummoningLabels::TriggerSummon),
            )
            .add_system(
                (default_human_dummy)
                    .label(SummoningLabels::DefaultSummon)
                    .after(SummoningLabels::NormalSummon),
            )
            .add_system(
                hands_attack_handler
                    .label(CombatLabels::WeaponHandler)
                    .after(CombatLabels::CacheAttack),
            );
    }
}

pub fn content_initialization(mut entity_data: ResMut<EntityDataResource>) {
    let entity_properties = EntityDataProperties {
        name: HUMAN_DUMMY_ENTITY_NAME.to_string(),
        id: entity_data.get_id_inc(),
        ..Default::default()
    };

    initialize_entity_data(&mut entity_data, entity_properties);

    let entity_properties = EntityDataProperties {
        name: HUMAN_MALE_ENTITY_NAME.to_string(),
        id: entity_data.get_id_inc(),
        ..Default::default()
    };

    initialize_entity_data(&mut entity_data, entity_properties);
}
