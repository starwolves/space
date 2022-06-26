use bevy_app::{App, Plugin};
use bevy_ecs::{schedule::ParallelSystemDescriptorCoercion, system::ResMut};

use crate::core::{
    entity::{
        functions::initialize_entity_data::initialize_entity_data,
        resources::{EntityDataProperties, EntityDataResource},
        spawn::SpawnEvent,
    },
    rigid_body::spawn::summon_rigid_body,
    StartupLabels, SummoningLabels,
};

use self::spawn::{
    default_human_dummy, entity_bundle::summon_base_human_male, summon_human_male,
    HumanMaleSummoner, HUMAN_DUMMY_ENTITY_NAME, HUMAN_MALE_ENTITY_NAME,
};

pub mod functions;
pub mod spawn;

pub struct HumanMalePlugin;

impl Plugin for HumanMalePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(content_initialization.before(StartupLabels::InitEntities))
            .add_system(
                summon_human_male
                    .before(SummoningLabels::TriggerSummon)
                    .label(SummoningLabels::DefaultSummon),
            )
            .add_system(
                (summon_base_human_male::<HumanMaleSummoner>).after(SummoningLabels::TriggerSummon),
            )
            .add_event::<SpawnEvent<HumanMaleSummoner>>()
            .add_system(
                (summon_rigid_body::<HumanMaleSummoner>).after(SummoningLabels::TriggerSummon),
            )
            .add_system((default_human_dummy).after(SummoningLabels::DefaultSummon));
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
