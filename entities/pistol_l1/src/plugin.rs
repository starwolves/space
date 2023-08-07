use basic_console_commands::register::{
    register_basic_console_commands_for_inventory_item_type,
    register_basic_console_commands_for_type,
};
use bevy::prelude::{App, FixedUpdate, IntoSystemConfigs, Plugin};
use combat::{
    laser_visuals::projectile_laser_visuals,
    melee_queries::melee_attack_handler,
    projectile_queries::projectile_attack_handler,
    sfx::{attack_sfx, health_combat_hit_result_sfx},
};
use entity::{
    entity_types::register_entity_type,
    spawn::{build_base_entities, SpawnItemSet},
};
use inventory::spawn_item::build_inventory_items;
use physics::spawn::build_rigid_bodies;
use resources::{
    is_server::is_server,
    sets::{CombatSet, MainSet},
};

use crate::pistol_l1::PistolL1;

use super::spawn::{build_pistols_l1, PistolL1Type};

pub struct PistolL1Plugin;

impl Plugin for PistolL1Plugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.add_systems(
                FixedUpdate,
                (
                    melee_attack_handler::<PistolL1>
                        .in_set(CombatSet::WeaponHandler)
                        .after(CombatSet::CacheAttack),
                    projectile_attack_handler::<PistolL1>
                        .in_set(CombatSet::WeaponHandler)
                        .after(CombatSet::CacheAttack),
                    attack_sfx::<PistolL1>
                        .after(CombatSet::WeaponHandler)
                        .after(CombatSet::Query),
                    health_combat_hit_result_sfx::<PistolL1>.after(CombatSet::FinalizeApplyDamage),
                    projectile_laser_visuals::<PistolL1>.after(CombatSet::Query),
                )
                    .in_set(MainSet::Update),
            );
        }
        register_entity_type::<PistolL1Type>(app);
        register_basic_console_commands_for_type::<PistolL1Type>(app);
        register_basic_console_commands_for_inventory_item_type::<PistolL1Type>(app);
        app.add_systems(
            FixedUpdate,
            (
                (build_base_entities::<PistolL1Type>).after(SpawnItemSet::SpawnHeldItem),
                (build_rigid_bodies::<PistolL1Type>).after(SpawnItemSet::SpawnHeldItem),
                (build_inventory_items::<PistolL1Type>).after(SpawnItemSet::SpawnHeldItem),
                build_pistols_l1::<PistolL1Type>.after(SpawnItemSet::SpawnHeldItem),
            )
                .in_set(MainSet::Update),
        );
    }
}
