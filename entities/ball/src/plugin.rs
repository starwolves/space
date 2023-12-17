use bevy::prelude::{App, FixedUpdate, IntoSystemConfigs, Plugin, Startup};

use entity::{
    base_mesh::link_base_mesh,
    entity_types::register_entity_type,
    loading::load_entity,
    spawn::{build_base_entities, SpawnItemSet},
};
use networking::messaging::{register_reliable_message, MessageSender};
use physics::spawn::build_rigid_bodies;
use resources::{
    correction::CorrectionSet,
    modes::is_server_mode,
    sets::{BuildingSet, MainSet},
};

use crate::{
    net::BallClientMessage,
    shoot_ball::{register_input, shoot_ball_client, shoot_ball_server},
};

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
                shoot_ball_server.in_set(BuildingSet::TriggerBuild),
            )
                .in_set(MainSet::Update),
        );

        if !is_server_mode(app) {
            app.add_systems(Startup, register_input);
            app.add_systems(
                FixedUpdate,
                (
                    shoot_ball_client,
                    link_base_mesh::<BallType>.after(SpawnItemSet::SpawnHeldItem),
                    load_entity::<BallType>
                        .before(SpawnItemSet::SpawnHeldItem)
                        .in_set(BuildingSet::TriggerBuild)
                        .in_set(CorrectionSet::Start),
                )
                    .in_set(MainSet::Update),
            );
        }
        register_reliable_message::<BallClientMessage>(app, MessageSender::Client, true);
    }
}
