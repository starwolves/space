use bevy::prelude::{App, IntoSystemDescriptor, Plugin, ResMut};
use combat::{
    laser_visuals::projectile_laser_visuals,
    melee_queries::melee_attack_handler,
    projectile_queries::projectile_attack_handler,
    sfx::{attack_sfx, health_combat_hit_result_sfx},
};
use entity::{
    entity_data::initialize_entity_data,
    entity_types::init_entity_type,
    meta::{EntityDataProperties, EntityDataResource},
    spawn::build_base_entities,
};
use inventory::spawn_item::build_inventory_items;
use physics::spawn::build_rigid_boies;
use resources::{
    is_server::is_server,
    labels::{BuildingLabels, CombatLabels, StartupLabels},
};

use crate::pistol_l1::{PistolL1, PISTOL_L1_ENTITY_NAME};

use super::spawn::{
    build_pistols_l1, build_raw_pistols_l1, default_build_pistols_l1, PistolL1Type,
};

pub struct PistolL1Plugin;

impl Plugin for PistolL1Plugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.add_startup_system(content_initialization.before(StartupLabels::InitEntities))
                .add_system(
                    (build_base_entities::<PistolL1Type>).after(BuildingLabels::TriggerBuild),
                )
                .add_system((build_rigid_boies::<PistolL1Type>).after(BuildingLabels::TriggerBuild))
                .add_system(
                    (build_inventory_items::<PistolL1Type>).after(BuildingLabels::TriggerBuild),
                )
                .add_system(build_pistols_l1::<PistolL1Type>.after(BuildingLabels::TriggerBuild))
                .add_system((build_raw_pistols_l1).after(BuildingLabels::TriggerBuild))
                .add_system(
                    (default_build_pistols_l1)
                        .label(BuildingLabels::DefaultBuild)
                        .after(BuildingLabels::NormalBuild),
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
                    health_combat_hit_result_sfx::<PistolL1>
                        .after(CombatLabels::FinalizeApplyDamage),
                )
                .add_system(projectile_laser_visuals::<PistolL1>.after(CombatLabels::Query));
        }
        init_entity_type::<PistolL1Type>(app);
    }
}

#[cfg(feature = "server")]
pub fn content_initialization(mut entity_data: ResMut<EntityDataResource>) {
    let entity_properties = EntityDataProperties {
        name: PISTOL_L1_ENTITY_NAME.to_string(),
        id: entity_data.get_id_inc(),
        ..Default::default()
    };

    initialize_entity_data(&mut entity_data, entity_properties);
}
