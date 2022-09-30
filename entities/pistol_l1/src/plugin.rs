use bevy::prelude::{App, ParallelSystemDescriptorCoercion, Plugin, ResMut};
use combat::{
    laser_visuals::projectile_laser_visuals,
    melee_queries::melee_attack_handler,
    projectile_queries::projectile_attack_handler,
    sfx::{attack_sfx, health_combat_hit_result_sfx},
};
use entity::{
    entity_data::initialize_entity_data,
    meta::{EntityDataProperties, EntityDataResource},
    spawn::{summon_base_entity, SpawnEvent},
};
use inventory_item::spawn::summon_inventory_item;
use rigid_body::spawn::summon_rigid_body;
use server::labels::{CombatLabels, StartupLabels, SummoningLabels};

use crate::pistol_l1::{PistolL1, PISTOL_L1_ENTITY_NAME};

use super::spawn::{
    default_summon_pistol_l1, summon_pistol_l1, summon_raw_pistol_l1, PistolL1Summoner,
};

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
            )
            .add_system(
                melee_attack_handler::<PistolL1>
                    .label(CombatLabels::WeaponHandler)
                    .after(CombatLabels::CacheAttack),
            )
            .add_system(
                projectile_attack_handler::<PistolL1>
                    .label(CombatLabels::WeaponHandler)
                    .after(CombatLabels::CacheAttack),
            )
            .add_system(
                attack_sfx::<PistolL1>
                    .after(CombatLabels::WeaponHandler)
                    .after(CombatLabels::Query),
            )
            .add_system(
                health_combat_hit_result_sfx::<PistolL1>.after(CombatLabels::FinalizeApplyDamage),
            )
            .add_system(projectile_laser_visuals::<PistolL1>.after(CombatLabels::Query));
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
