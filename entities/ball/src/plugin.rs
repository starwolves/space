use bevy::prelude::{App, FixedUpdate, IntoSystemConfigs, Plugin, Startup};

use entity::{
    base_mesh::link_base_mesh,
    entity_types::register_entity_type,
    spawn::{build_base_entities, SpawnItemSet},
};
use physics::spawn::build_rigid_bodies;
use resources::{
    is_server::is_server,
    sets::{BuildingSet, MainSet},
};

use crate::shoot_ball::{register_input, shoot_ball};

use super::spawn::{build_balls, BallType};

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        register_entity_type::<BallType>(app);
        app.add_systems(
            FixedUpdate,
            (
                build_balls::<BallType>.after(SpawnItemSet::SpawnHeldItem),
                (build_base_entities::<BallType>).after(SpawnItemSet::SpawnHeldItem),
                (build_rigid_bodies::<BallType>).after(SpawnItemSet::SpawnHeldItem),
            )
                .in_set(MainSet::Update),
        );

        if !is_server() {
            app.add_systems(Startup, register_input);
            app.add_systems(
                FixedUpdate,
                (
                    shoot_ball.in_set(BuildingSet::TriggerBuild),
                    link_base_mesh::<BallType>.after(SpawnItemSet::SpawnHeldItem),
                )
                    .in_set(MainSet::Update),
            );
        }
    }
}
