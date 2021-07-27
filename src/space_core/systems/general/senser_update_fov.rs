use bevy::{prelude::{Query, ResMut}};
use bevy_rapier3d::prelude::RigidBodyPosition;
use doryen_fov::FovAlgorithm;

use crate::space_core::{components::senser::Senser, functions::gridmap::gridmap_functions::world_to_cell_id, resources::{doryen_fov::{DoryenMap, to_doryen_coordinates}, precalculated_fov_data::Vec2Int}};


pub fn senser_update_fov(
    mut senser_entities : Query<(&mut Senser, &RigidBodyPosition)>,
    mut map : ResMut<DoryenMap>,
) {

    for (mut senser_component, rigid_body_position_component) in senser_entities.iter_mut() {

        let senser_cell_id_3 = world_to_cell_id(rigid_body_position_component.position.translation.into());
        let senser_cell_id = Vec2Int {
            x : senser_cell_id_3.x,
            y : senser_cell_id_3.z,
        };

        if senser_component.cell_id != senser_cell_id {


            senser_component.cell_id = senser_cell_id;


            // 240000 ns. 1/4th of a ms. 4x/ms (expensive.)
            senser_component.fov.clear_fov();
            let coords = to_doryen_coordinates(senser_cell_id.x, senser_cell_id.y);
            senser_component.fov.compute_fov(&mut map.map, coords.0, coords.1, 20, false);

            
        }
    }

}
