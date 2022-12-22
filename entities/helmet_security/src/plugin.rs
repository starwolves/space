use bevy::prelude::{App, IntoSystemDescriptor, Plugin, ResMut};
use combat::{
    melee_queries::melee_attack_handler,
    sfx::{attack_sfx, health_combat_hit_result_sfx},
};
use entity::{
    entity_data::initialize_entity_data,
    meta::{EntityDataProperties, EntityDataResource},
    spawn::{build_base_entities, SpawnEntity},
};
use inventory::spawn_item::build_inventory_items;
use physics::spawn::build_rigid_boies;
use resources::{
    is_server::is_server,
    labels::{BuildingLabels, CombatLabels, StartupLabels},
};

use crate::helmet::{Helmet, HELMET_SECURITY_ENTITY_NAME};

use super::spawn::{
    build_helmets, build_raw_helmets, default_build_helmets_security, HelmetBuilder,
};

pub struct HelmetsPlugin;

impl Plugin for HelmetsPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.add_startup_system(content_initialization.before(StartupLabels::InitEntities))
                .add_system(build_helmets::<HelmetBuilder>.after(BuildingLabels::TriggerBuild))
                .add_system(
                    (build_base_entities::<HelmetBuilder>).after(BuildingLabels::TriggerBuild),
                )
                .add_system(
                    (build_rigid_boies::<HelmetBuilder>).after(BuildingLabels::TriggerBuild),
                )
                .add_system(
                    (build_inventory_items::<HelmetBuilder>).after(BuildingLabels::TriggerBuild),
                )
                .add_system((build_raw_helmets).after(BuildingLabels::TriggerBuild))
                .add_event::<SpawnEntity<HelmetBuilder>>()
                .add_system(
                    (default_build_helmets_security)
                        .label(BuildingLabels::DefaultBuild)
                        .after(BuildingLabels::NormalBuild),
                )
                .add_system(
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
    }
}

pub fn content_initialization(mut entity_data: ResMut<EntityDataResource>) {
    let entity_properties = EntityDataProperties {
        name: HELMET_SECURITY_ENTITY_NAME.to_string(),
        id: entity_data.get_id_inc(),
        ..Default::default()
    };

    initialize_entity_data(&mut entity_data, entity_properties);
}
