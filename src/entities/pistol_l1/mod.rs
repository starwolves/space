use bevy_app::{App, Plugin};
use bevy_ecs::{schedule::ParallelSystemDescriptorCoercion, system::ResMut};

use crate::core::{
    entity::{
        functions::initialize_entity_data::initialize_entity_data,
        resources::{EntityDataProperties, EntityDataResource},
        spawn::{summon_base_entity, SpawnEvent},
    },
    inventory_item::spawn::summon_inventory_item,
    rigid_body::spawn::summon_rigid_body,
    StartupLabels, SummoningLabels,
};

use self::spawn::{
    default_summon_pistol_l1, summon_pistol_l1, summon_raw_pistol_l1, PistolL1Summoner,
    PISTOL_L1_ENTITY_NAME,
};

pub mod components;
pub mod spawn;

pub struct PistolL1Plugin;

impl Plugin for PistolL1Plugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(content_initialization.before(StartupLabels::InitEntities))
            .add_system(
                (summon_base_entity::<PistolL1Summoner>).after(SummoningLabels::TriggerSummon),
            )
            .add_system(
                (summon_rigid_body::<PistolL1Summoner>).after(SummoningLabels::TriggerSummon),
            )
            .add_system(
                (summon_inventory_item::<PistolL1Summoner>).after(SummoningLabels::TriggerSummon),
            )
            .add_system(summon_pistol_l1::<PistolL1Summoner>.after(SummoningLabels::TriggerSummon))
            .add_system((summon_raw_pistol_l1).after(SummoningLabels::TriggerSummon))
            .add_event::<SpawnEvent<PistolL1Summoner>>()
            .add_system(
                (default_summon_pistol_l1)
                    .label(SummoningLabels::DefaultSummon)
                    .after(SummoningLabels::NormalSummon),
            );
    }
}

pub fn content_initialization(mut entity_data: ResMut<EntityDataResource>) {
    let entity_properties = EntityDataProperties {
        name: PISTOL_L1_ENTITY_NAME.to_string(),
        id: entity_data.get_id_inc(),
        ..Default::default()
    };

    initialize_entity_data(&mut entity_data, entity_properties);
}
