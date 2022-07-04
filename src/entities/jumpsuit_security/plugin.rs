use bevy::prelude::{App, ParallelSystemDescriptorCoercion, Plugin, ResMut};

use crate::core::{
    entity::{
        entity_data::{initialize_entity_data, EntityDataProperties, EntityDataResource},
        spawn::{summon_base_entity, SpawnEvent},
    },
    inventory_item::spawn::summon_inventory_item,
    rigid_body::spawn::summon_rigid_body,
    space_plugin::plugin::{StartupLabels, SummoningLabels},
};

use super::spawn::{
    default_summon_jumpsuit, summon_jumpsuit, summon_raw_jumpsuit, JumpsuitSummoner,
    JUMPSUIT_SECURITY_ENTITY_NAME,
};

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
