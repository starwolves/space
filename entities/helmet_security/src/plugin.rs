use basic_console_commands::register::{
    register_basic_console_commands_for_inventory_item_type,
    register_basic_console_commands_for_type,
};
use bevy::prelude::{App, IntoSystemDescriptor, Plugin};
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

use crate::helmet::Helmet;

use super::spawn::{build_helmets, HelmetType};

pub struct HelmetsPlugin;

impl Plugin for HelmetsPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.add_system(
                melee_attack_handler::<Helmet>
                    .label(CombatLabels::WeaponHandler)
                    .after(CombatLabels::CacheAttack),
            )
            .add_system(
                attack_sfx::<Helmet>
                    .after(CombatLabels::WeaponHandler)
                    .after(CombatLabels::Query),
            )
            .add_system(
                health_combat_hit_result_sfx::<Helmet>.after(CombatLabels::FinalizeApplyDamage),
            );
        }
        register_entity_type::<HelmetType>(app);
        register_basic_console_commands_for_type::<HelmetType>(app);
        register_basic_console_commands_for_inventory_item_type::<HelmetType>(app);
        app.add_system(build_helmets::<HelmetType>.after(BuildingLabels::TriggerBuild))
            .add_system((build_base_entities::<HelmetType>).after(BuildingLabels::TriggerBuild))
            .add_system((build_rigid_bodies::<HelmetType>).after(BuildingLabels::TriggerBuild))
            .add_system((build_inventory_items::<HelmetType>).after(BuildingLabels::TriggerBuild));
    }
}
