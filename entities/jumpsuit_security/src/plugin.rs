use bevy::prelude::{App, IntoSystemDescriptor, Plugin, ResMut};
use combat::{
    melee_queries::melee_attack_handler,
    sfx::{attack_sfx, health_combat_hit_result_sfx},
};
use entity::{
    entity_data::initialize_entity_data,
    entity_types::init_entity_type,
    meta::{EntityDataProperties, EntityDataResource},
    spawn::{build_base_entities, SpawnEntity},
};
use inventory::spawn_item::build_inventory_items;
use physics::spawn::build_rigid_boies;
use resources::{
    is_server::is_server,
    labels::{BuildingLabels, CombatLabels, StartupLabels},
};

use crate::jumpsuit::{Jumpsuit, JUMPSUIT_SECURITY_ENTITY_NAME};

use super::spawn::{build_jumpsuits, build_raw_jumpsuits, default_build_jumpsuits, JumpsuitType};

pub struct JumpsuitsPlugin;

impl Plugin for JumpsuitsPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.add_startup_system(content_initialization.before(StartupLabels::InitEntities))
                .add_system(build_jumpsuits::<JumpsuitType>.after(BuildingLabels::TriggerBuild))
                .add_system(
                    (build_base_entities::<JumpsuitType>).after(BuildingLabels::TriggerBuild),
                )
                .add_system((build_rigid_boies::<JumpsuitType>).after(BuildingLabels::TriggerBuild))
                .add_system((build_raw_jumpsuits).after(BuildingLabels::TriggerBuild))
                .add_system(
                    (build_inventory_items::<JumpsuitType>).after(BuildingLabels::TriggerBuild),
                )
                .add_event::<SpawnEntity<JumpsuitType>>()
                .add_system(
                    (default_build_jumpsuits)
                        .label(BuildingLabels::DefaultBuild)
                        .after(BuildingLabels::NormalBuild),
                )
                .add_system(
                    melee_attack_handler::<Jumpsuit>
                        .label(CombatLabels::WeaponHandler)
                        .after(CombatLabels::CacheAttack),
                )
                .add_system(
                    attack_sfx::<Jumpsuit>
                        .after(CombatLabels::WeaponHandler)
                        .after(CombatLabels::Query),
                )
                .add_system(
                    health_combat_hit_result_sfx::<Jumpsuit>
                        .after(CombatLabels::FinalizeApplyDamage),
                );
        }
        init_entity_type::<JumpsuitType>(app);
    }
}

pub fn content_initialization(mut entity_data: ResMut<EntityDataResource>) {
    let entity_properties = EntityDataProperties {
        name: JUMPSUIT_SECURITY_ENTITY_NAME.to_string(),
        id: entity_data.get_id_inc(),
        ..Default::default()
    };

    initialize_entity_data(&mut entity_data, entity_properties);
}
