use api::{
    data::Vec2Int,
    gridmap::{get_atmos_index, GridMapType, GridmapMain, RemoveCell},
};
use bevy::prelude::{EventReader, Res, ResMut};

use crate::diffusion::{AtmosphericsResource, EffectType, VACUUM_ATMOSEFFECT};

/// When a cell gets removed, ie deconstruct event on floor or wall tile, update atmospherics.
pub(crate) fn remove_cell_atmos_event(
    mut deconstruct_cell_events: EventReader<RemoveCell>,
    gridmap_main: Res<GridmapMain>,
    mut atmospherics_resource: ResMut<AtmosphericsResource>,
) {
    for event in deconstruct_cell_events.iter() {
        match event.gridmap_type {
            GridMapType::Main => {
                let mut atmospherics = atmospherics_resource
                    .atmospherics
                    .get_mut(get_atmos_index(Vec2Int {
                        x: event.id.x,
                        y: event.id.z,
                    }))
                    .unwrap();

                if event.id.y == 0 {
                    atmospherics.blocked = false;
                    atmospherics.forces_push_up = false;
                } else {
                    let mut upper_id = event.id.clone();
                    upper_id.y = 0;

                    // Add vacuum flag to atmos.
                    match gridmap_main.grid_data.get(&upper_id) {
                        Some(_) => {}
                        None => {
                            atmospherics
                                .effects
                                .insert(EffectType::Floorless, VACUUM_ATMOSEFFECT);
                        }
                    }
                }
            }
            GridMapType::Details1 => {}
        }
    }
}
