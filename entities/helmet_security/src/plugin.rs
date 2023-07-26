use basic_console_commands::register::{
    register_basic_console_commands_for_inventory_item_type,
    register_basic_console_commands_for_type,
};
use bevy::prelude::{App, FixedUpdate, IntoSystemConfigs, Plugin};
use combat::{
    melee_queries::melee_attack_handler,
    sfx::{attack_sfx, health_combat_hit_result_sfx},
};
use entity::{entity_types::register_entity_type, spawn::build_base_entities};
use inventory::spawn_item::build_inventory_items;
use physics::spawn::build_rigid_bodies;
use resources::{
    is_server::is_server,
    sets::{BuildingSet, CombatSet, MainSet},
};

use crate::helmet::Helmet;

use super::spawn::{build_helmets, HelmetType};

pub struct HelmetsPlugin;

impl Plugin for HelmetsPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.add_systems(
                FixedUpdate,
                (
                    melee_attack_handler::<Helmet>
                        .in_set(CombatSet::WeaponHandler)
                        .after(CombatSet::CacheAttack),
                    attack_sfx::<Helmet>
                        .after(CombatSet::WeaponHandler)
                        .after(CombatSet::Query),
                    health_combat_hit_result_sfx::<Helmet>.after(CombatSet::FinalizeApplyDamage),
                )
                    .in_set(MainSet::Update),
            );
        }
        register_entity_type::<HelmetType>(app);
        register_basic_console_commands_for_type::<HelmetType>(app);
        register_basic_console_commands_for_inventory_item_type::<HelmetType>(app);
        app.add_systems(
            FixedUpdate,
            (
                build_helmets::<HelmetType>.after(BuildingSet::TriggerBuild),
                (build_base_entities::<HelmetType>).after(BuildingSet::TriggerBuild),
                (build_rigid_bodies::<HelmetType>).after(BuildingSet::TriggerBuild),
                (build_inventory_items::<HelmetType>).after(BuildingSet::TriggerBuild),
            )
                .in_set(MainSet::Update),
        );
    }
}
