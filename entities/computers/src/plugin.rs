use bevy::prelude::{App, IntoSystemDescriptor, Plugin};
use combat::sfx::health_combat_hit_result_sfx;
use entity::register::register_entity_type;
use entity::spawn::build_base_entities;
use physics::spawn::build_rigid_bodies;
use resources::{
    is_server::is_server,
    labels::{BuildingLabels, CombatLabels},
};

use crate::computer::Computer;

use super::{
    computer::computer_added,
    spawn::{build_computers, build_raw_computers, default_build_computers, ComputerType},
};

pub struct ComputersPlugin;

impl Plugin for ComputersPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.add_system(computer_added).add_system(
                health_combat_hit_result_sfx::<Computer>.after(CombatLabels::FinalizeApplyDamage),
            );
        }
        register_entity_type::<ComputerType>(app);
        app.add_system(build_computers::<ComputerType>.after(BuildingLabels::TriggerBuild))
            .add_system((build_base_entities::<ComputerType>).after(BuildingLabels::TriggerBuild))
            .add_system((build_rigid_bodies::<ComputerType>).after(BuildingLabels::TriggerBuild))
            .add_system((build_raw_computers).after(BuildingLabels::TriggerBuild))
            .add_system(
                (default_build_computers)
                    .label(BuildingLabels::DefaultBuild)
                    .after(BuildingLabels::NormalBuild),
            );
    }
}
