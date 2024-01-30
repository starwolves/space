use bevy::prelude::{App, IntoSystemConfigs, Plugin};
use combat::sfx::health_combat_hit_result_sfx;
use entity::entity_types::register_entity_type;
use entity::spawn::build_base_entities;
use physics::spawn::build_rigid_bodies;
use resources::modes::is_server_mode;
use resources::ordering::{BuildingSet, CombatSet, PreUpdate, Update};

use crate::computer::Computer;

use super::{
    computer::computer_added,
    spawn::{build_computers, ComputerType},
};

pub struct ComputersPlugin;

impl Plugin for ComputersPlugin {
    fn build(&self, app: &mut App) {
        if is_server_mode(app) {
            app.add_systems(
                Update,
                (
                    health_combat_hit_result_sfx::<Computer>.after(CombatSet::FinalizeApplyDamage),
                    computer_added,
                ),
            );
        }
        register_entity_type::<ComputerType>(app);
        app.add_systems(
            PreUpdate,
            (
                (build_rigid_bodies::<ComputerType>).in_set(BuildingSet::NormalBuild),
                build_computers::<ComputerType>.in_set(BuildingSet::NormalBuild),
                (build_base_entities::<ComputerType>).in_set(BuildingSet::NormalBuild),
            ),
        );
    }
}
