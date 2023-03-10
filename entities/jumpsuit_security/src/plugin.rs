use basic_console_commands::register::{
    register_basic_console_commands_for_inventory_item_type,
    register_basic_console_commands_for_type,
};
use bevy::prelude::{App, IntoSystemConfig, Plugin};
use combat::{
    melee_queries::melee_attack_handler,
    sfx::{attack_sfx, health_combat_hit_result_sfx},
};
use entity::{entity_types::register_entity_type, spawn::build_base_entities};
use inventory::spawn_item::build_inventory_items;
use physics::spawn::build_rigid_bodies;
use resources::{
    is_server::is_server,
    labels::{BuildingLabels, CombatLabels},
};

use crate::jumpsuit::Jumpsuit;

use super::spawn::{build_jumpsuits, JumpsuitType};

pub struct JumpsuitsPlugin;

impl Plugin for JumpsuitsPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.add_system(
                melee_attack_handler::<Jumpsuit>
                    .in_set(CombatLabels::WeaponHandler)
                    .after(CombatLabels::CacheAttack),
            )
            .add_system(
                attack_sfx::<Jumpsuit>
                    .after(CombatLabels::WeaponHandler)
                    .after(CombatLabels::Query),
            )
            .add_system(
                health_combat_hit_result_sfx::<Jumpsuit>.after(CombatLabels::FinalizeApplyDamage),
            );
        }
        register_entity_type::<JumpsuitType>(app);
        register_basic_console_commands_for_type::<JumpsuitType>(app);
        register_basic_console_commands_for_inventory_item_type::<JumpsuitType>(app);
        app.add_system(build_jumpsuits::<JumpsuitType>.after(BuildingLabels::TriggerBuild))
            .add_system((build_base_entities::<JumpsuitType>).after(BuildingLabels::TriggerBuild))
            .add_system((build_rigid_bodies::<JumpsuitType>).after(BuildingLabels::TriggerBuild))
            .add_system(
                (build_inventory_items::<JumpsuitType>).after(BuildingLabels::TriggerBuild),
            );
    }
}
