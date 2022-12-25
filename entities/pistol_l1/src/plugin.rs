use basic_console_commands::register::{
    register_basic_console_commands_for_inventory_item_type,
    register_basic_console_commands_for_type,
};
use bevy::prelude::{App, IntoSystemDescriptor, Plugin};
use combat::{
    laser_visuals::projectile_laser_visuals,
    melee_queries::melee_attack_handler,
    projectile_queries::projectile_attack_handler,
    sfx::{attack_sfx, health_combat_hit_result_sfx},
};
use entity::{register::register_entity_type, spawn::build_base_entities};
use inventory::spawn_item::build_inventory_items;
use physics::spawn::build_rigid_bodies;
use resources::{
    is_server::is_server,
    labels::{BuildingLabels, CombatLabels},
};

use crate::pistol_l1::PistolL1;

use super::spawn::{build_pistols_l1, build_raw_pistols_l1, PistolL1Type};

pub struct PistolL1Plugin;

impl Plugin for PistolL1Plugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.add_system(
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
        register_entity_type::<PistolL1Type>(app);
        register_basic_console_commands_for_type::<PistolL1Type>(app);
        register_basic_console_commands_for_inventory_item_type::<PistolL1Type>(app);
        app.add_system((build_base_entities::<PistolL1Type>).after(BuildingLabels::TriggerBuild))
            .add_system((build_rigid_bodies::<PistolL1Type>).after(BuildingLabels::TriggerBuild))
            .add_system((build_inventory_items::<PistolL1Type>).after(BuildingLabels::TriggerBuild))
            .add_system(build_pistols_l1::<PistolL1Type>.after(BuildingLabels::TriggerBuild))
            .add_system((build_raw_pistols_l1).after(BuildingLabels::TriggerBuild));
    }
}
