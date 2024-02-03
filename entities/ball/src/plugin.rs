use bevy::prelude::{App, IntoSystemConfigs, Plugin, Startup};

use entity::{
    base_mesh::link_base_mesh, entity_types::register_entity_type, loading::load_entity,
    spawn::build_base_entities,
};
use networking::messaging::{register_reliable_message, MessageSender};
use physics::spawn::build_rigid_bodies;
use resources::{
    modes::is_server_mode,
    ordering::{BuildingSet, PreUpdate, Update},
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
            PreUpdate,
            (
                build_balls::<BallType>.in_set(BuildingSet::NormalBuild),
                (build_base_entities::<BallType>).in_set(BuildingSet::NormalBuild),
                (build_rigid_bodies::<BallType>).in_set(BuildingSet::NormalBuild),
            ),
        );

        if !is_server_mode(app) {
            app.add_systems(Startup, register_input);
            app.add_systems(Update, (shoot_ball_client,));
            app.add_systems(
                PreUpdate,
                (
                    link_base_mesh::<BallType>.in_set(BuildingSet::NormalBuild),
                    load_entity::<BallType>
                        .before(BuildingSet::NormalBuild)
                        .in_set(BuildingSet::TriggerBuild),
                ),
            );
        } else {
            app.add_systems(Update, (shoot_ball_server,));
        }
        register_reliable_message::<BallClientMessage>(app, MessageSender::Client, true);
    }
}
