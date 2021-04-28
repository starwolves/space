use bevy::prelude::{Entity, Query, ResMut};
use bevy_networking_turbulence::NetworkResource;
use bevy_rapier3d::physics::RigidBodyHandleComponent;

use crate::space_core::components::{connected_player::ConnectedPlayer, visible::Visible, visible_checker::VisibleChecker};

pub fn broadcast_interpolation_transforms (
    mut net: ResMut<NetworkResource>,
    query_interpolated_entities : Query<(Entity, &Visible, &RigidBodyHandleComponent)>,
    query_visible_checkers : Query<(&VisibleChecker, &ConnectedPlayer)>
) {

    

}