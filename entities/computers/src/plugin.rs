use bevy::prelude::{App, FixedUpdate, IntoSystemConfigs, Plugin};
use combat::sfx::health_combat_hit_result_sfx;
use entity::spawn::build_base_entities;
use entity::{entity_types::register_entity_type, spawn::SpawnItemSet};
use physics::spawn::build_rigid_bodies;
use resources::{
    is_server::is_server,
    sets::{CombatSet, MainSet},
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
                FixedUpdate,
                (
                    health_combat_hit_result_sfx::<Computer>.after(CombatSet::FinalizeApplyDamage),
                    computer_added,
                )
                    .in_set(MainSet::Update),
            );
        }
        register_entity_type::<ComputerType>(app);
        app.add_systems(
            FixedUpdate,
            (
                (build_rigid_bodies::<ComputerType>).after(SpawnItemSet::SpawnHeldItem),
                build_computers::<ComputerType>.after(SpawnItemSet::SpawnHeldItem),
                (build_base_entities::<ComputerType>).after(SpawnItemSet::SpawnHeldItem),
            )
                .in_set(MainSet::Update),
        );
    }
}
