use basic_console_commands::register::{
    register_basic_console_commands_for_inventory_item_type,
    register_basic_console_commands_for_type,
};
use bevy::prelude::{App, IntoSystemConfigs, Plugin};
use combat::{
    melee_queries::melee_attack_handler,
    sfx::{attack_sfx, health_combat_hit_result_sfx},
};
use entity::{entity_types::register_entity_type, spawn::build_base_entities};
use inventory::spawn_item::build_inventory_items;
use physics::spawn::build_rigid_bodies;
use resources::{
    modes::is_server_mode,
    ordering::{BuildingSet, CombatSet, PreUpdate, Update},
};

use crate::jumpsuit::Jumpsuit;

use super::spawn::{build_jumpsuits, JumpsuitType};

pub struct JumpsuitsPlugin;

impl Plugin for JumpsuitsPlugin {
    fn build(&self, app: &mut App) {
        if is_server_mode(app) {
            app.add_systems(
                Update,
                (
                    melee_attack_handler::<Jumpsuit>
                        .in_set(CombatSet::WeaponHandler)
                        .after(CombatSet::CacheAttack),
                    attack_sfx::<Jumpsuit>
                        .after(CombatSet::WeaponHandler)
                        .after(CombatSet::Query),
                    health_combat_hit_result_sfx::<Jumpsuit>.after(CombatSet::FinalizeApplyDamage),
                ),
            );
        }
        register_entity_type::<JumpsuitType>(app);
        register_basic_console_commands_for_type::<JumpsuitType>(app);
        register_basic_console_commands_for_inventory_item_type::<JumpsuitType>(app);
        app.add_systems(
            PreUpdate,
            (
                build_jumpsuits::<JumpsuitType>.in_set(BuildingSet::NormalBuild),
                (build_base_entities::<JumpsuitType>).in_set(BuildingSet::NormalBuild),
                (build_rigid_bodies::<JumpsuitType>).in_set(BuildingSet::NormalBuild),
                (build_inventory_items::<JumpsuitType>).in_set(BuildingSet::NormalBuild),
            ),
        );
    }
}
