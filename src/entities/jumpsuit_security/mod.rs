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
    default_summon_jumpsuit, summon_jumpsuit, summon_raw_jumpsuit, JumpsuitSummoner,
    JUMPSUIT_SECURITY_ENTITY_NAME,
};

pub mod components;
pub mod spawn;

pub struct JumpsuitsPlugin;

impl Plugin for JumpsuitsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(content_initialization.before(StartupLabels::InitEntities))
            .add_system(summon_jumpsuit::<JumpsuitSummoner>.after(SummoningLabels::TriggerSummon))
            .add_system(
                (summon_base_entity::<JumpsuitSummoner>).after(SummoningLabels::TriggerSummon),
            )
            .add_system(
                (summon_rigid_body::<JumpsuitSummoner>).after(SummoningLabels::TriggerSummon),
            )
            .add_system((summon_raw_jumpsuit).after(SummoningLabels::TriggerSummon))
            .add_system(
                (summon_inventory_item::<JumpsuitSummoner>).after(SummoningLabels::TriggerSummon),
            )
            .add_event::<SpawnEvent<JumpsuitSummoner>>()
            .add_system(
                (default_summon_jumpsuit)
                    .label(SummoningLabels::DefaultSummon)
                    .after(SummoningLabels::NormalSummon),
            );
    }
}

pub fn content_initialization(mut entity_data: ResMut<EntityDataResource>) {
    let entity_properties = EntityDataProperties {
        name: JUMPSUIT_SECURITY_ENTITY_NAME.to_string(),
        id: entity_data.get_id_inc(),
        ..Default::default()
    };

    initialize_entity_data(&mut entity_data, entity_properties);
}
