use bevy::prelude::{App, IntoSystemDescriptor, Plugin, ResMut};
use combat::sfx::health_combat_hit_result_sfx;
use entity::{
    entity_data::initialize_entity_data,
    meta::{EntityDataProperties, EntityDataResource},
    spawn::{build_base_entities, SpawnEntity},
};
use physics::spawn::build_rigid_boies;
use resources::{
    is_server::is_server,
    labels::{BuildingLabels, CombatLabels, StartupLabels},
};

use crate::computer::Computer;

use super::{
    computer::computer_added,
    spawn::{
        build_computers, build_raw_computers, default_build_computers, ComputerBuilder,
        BRIDGE_COMPUTER_ENTITY_NAME,
    },
};

pub struct ComputersPlugin;

impl Plugin for ComputersPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.add_system(computer_added).add_system(
                health_combat_hit_result_sfx::<Computer>.after(CombatLabels::FinalizeApplyDamage),
            );
        }
        app.add_event::<SpawnEntity<ComputerBuilder>>()
            .add_startup_system(content_initialization.before(StartupLabels::BuildGridmap))
            .add_system(build_computers::<ComputerBuilder>.after(BuildingLabels::TriggerBuild))
            .add_system(
                (build_base_entities::<ComputerBuilder>).after(BuildingLabels::TriggerBuild),
            )
            .add_system((build_rigid_boies::<ComputerBuilder>).after(BuildingLabels::TriggerBuild))
            .add_system((build_raw_computers).after(BuildingLabels::TriggerBuild))
            .add_system(
                (default_build_computers)
                    .label(BuildingLabels::DefaultBuild)
                    .after(BuildingLabels::NormalBuild),
            );
    }
}

#[cfg(feature = "server")]
fn content_initialization(mut entity_data: ResMut<EntityDataResource>) {
    let entity_properties = EntityDataProperties {
        name: BRIDGE_COMPUTER_ENTITY_NAME.to_string(),
        id: entity_data.get_id_inc(),
        ..Default::default()
    };
    initialize_entity_data(&mut entity_data, entity_properties);
}
