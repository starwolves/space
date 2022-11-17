use std::env;

use bevy::prelude::{App, ParallelSystemDescriptorCoercion, Plugin, ResMut};
use combat::{
    melee_queries::melee_attack_handler,
    sfx::{attack_sfx, health_combat_hit_result_sfx},
};
use entity::{
    entity_data::initialize_entity_data,
    meta::{EntityDataProperties, EntityDataResource},
    spawn::{summon_base_entity, SpawnEvent},
};
use inventory_item::spawn::summon_inventory_item;
use resources::labels::{CombatLabels, StartupLabels, SummoningLabels};
use rigid_body::spawn::summon_rigid_body;

use crate::jumpsuit::{Jumpsuit, JUMPSUIT_SECURITY_ENTITY_NAME};

use super::spawn::{
    default_summon_jumpsuit, summon_jumpsuit, summon_raw_jumpsuit, JumpsuitSummoner,
};

pub struct JumpsuitsPlugin;

impl Plugin for JumpsuitsPlugin {
    fn build(&self, app: &mut App) {
        if env::var("CARGO_MANIFEST_DIR").unwrap().ends_with("server") {
            app.add_startup_system(content_initialization.before(StartupLabels::InitEntities))
                .add_system(
                    summon_jumpsuit::<JumpsuitSummoner>.after(SummoningLabels::TriggerSummon),
                )
                .add_system(
                    (summon_base_entity::<JumpsuitSummoner>).after(SummoningLabels::TriggerSummon),
                )
                .add_system(
                    (summon_rigid_body::<JumpsuitSummoner>).after(SummoningLabels::TriggerSummon),
                )
                .add_system((summon_raw_jumpsuit).after(SummoningLabels::TriggerSummon))
                .add_system(
                    (summon_inventory_item::<JumpsuitSummoner>)
                        .after(SummoningLabels::TriggerSummon),
                )
                .add_event::<SpawnEvent<JumpsuitSummoner>>()
                .add_system(
                    (default_summon_jumpsuit)
                        .label(SummoningLabels::DefaultSummon)
                        .after(SummoningLabels::NormalSummon),
                )
                .add_system(
                    melee_attack_handler::<Jumpsuit>
                        .label(CombatLabels::WeaponHandler)
                        .after(CombatLabels::CacheAttack),
                )
                .add_system(
                    attack_sfx::<Jumpsuit>
                        .after(CombatLabels::WeaponHandler)
                        .after(CombatLabels::Query),
                )
                .add_system(
                    health_combat_hit_result_sfx::<Jumpsuit>
                        .after(CombatLabels::FinalizeApplyDamage),
                );
        }
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
