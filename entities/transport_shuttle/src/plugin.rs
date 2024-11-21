use bevy::{
    app::{App, Plugin},
    prelude::IntoSystemConfigs,
};
use entity::{
    base_mesh::link_base_mesh, entity_types::register_entity_type, loading::load_entity,
    spawn::build_base_entities,
};
use physics::spawn::build_rigid_bodies;
use resources::{
    modes::is_server_mode,
    ordering::{BuildingSet, PreUpdate},
};

use crate::spawn::{build_transport_shuttle, TransportShuttleType};

pub struct TransportShuttlePlugin;

impl Plugin for TransportShuttlePlugin {
    fn build(&self, app: &mut App) {
        if !is_server_mode(app) {
            app.add_systems(
                PreUpdate,
                (
                    load_entity::<TransportShuttleType>
                        .before(BuildingSet::NormalBuild)
                        .in_set(BuildingSet::TriggerBuild),
                    link_base_mesh::<TransportShuttleType>.in_set(BuildingSet::NormalBuild),
                ),
            );
        }

        app.add_systems(
            PreUpdate,
            (
                build_transport_shuttle::<TransportShuttleType>.after(BuildingSet::NormalBuild),
                (build_rigid_bodies::<TransportShuttleType>).after(BuildingSet::NormalBuild),
                (build_base_entities::<TransportShuttleType>).after(BuildingSet::NormalBuild),
            ),
        );
        register_entity_type::<TransportShuttleType>(app);
    }
}
