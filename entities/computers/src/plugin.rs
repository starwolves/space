use bevy::prelude::{App, IntoSystemConfigs, Plugin, Update};
use combat::sfx::health_combat_hit_result_sfx;
use entity::entity_types::register_entity_type;
use entity::spawn::build_base_entities;
use physics::spawn::build_rigid_bodies;
use resources::{
    is_server::is_server,
    labels::{BuildingLabels, CombatLabels},
};

use crate::computer::Computer;

use super::{
    computer::computer_added,
    spawn::{build_computers, ComputerType},
};

pub struct ComputersPlugin;

impl Plugin for ComputersPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.add_systems(
                Update,
                (
                    health_combat_hit_result_sfx::<Computer>
                        .after(CombatLabels::FinalizeApplyDamage),
                    computer_added,
                ),
            );
        }
        register_entity_type::<ComputerType>(app);
        app.add_systems(
            Update,
            (
                (build_rigid_bodies::<ComputerType>).after(BuildingLabels::TriggerBuild),
                build_computers::<ComputerType>.after(BuildingLabels::TriggerBuild),
                (build_base_entities::<ComputerType>).after(BuildingLabels::TriggerBuild),
            ),
        );
    }
}
