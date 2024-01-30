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

use crate::helmet::Helmet;

use super::spawn::{build_helmets, HelmetType};

pub struct HelmetsPlugin;

impl Plugin for HelmetsPlugin {
    fn build(&self, app: &mut App) {
        if is_server_mode(app) {
            app.add_systems(
                Update,
                (
                    melee_attack_handler::<Helmet>
                        .in_set(CombatSet::WeaponHandler)
                        .after(CombatSet::CacheAttack),
                    attack_sfx::<Helmet>
                        .after(CombatSet::WeaponHandler)
                        .after(CombatSet::Query),
                    health_combat_hit_result_sfx::<Helmet>.after(CombatSet::FinalizeApplyDamage),
                ),
            );
        }
        register_entity_type::<HelmetType>(app);
        register_basic_console_commands_for_type::<HelmetType>(app);
        register_basic_console_commands_for_inventory_item_type::<HelmetType>(app);
        app.add_systems(
            PreUpdate,
            (
                build_helmets::<HelmetType>.in_set(BuildingSet::NormalBuild),
                (build_base_entities::<HelmetType>).in_set(BuildingSet::NormalBuild),
                (build_rigid_bodies::<HelmetType>).in_set(BuildingSet::NormalBuild),
                (build_inventory_items::<HelmetType>).in_set(BuildingSet::NormalBuild),
            ),
        );
    }
}
