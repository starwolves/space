use bevy::prelude::{EventReader, Res, ResMut};
use gridmap::grid::{Gridmap, RemoveCell};
use math::grid::Vec2Int;

use crate::diffusion::{get_atmos_index, AtmosphericsResource, EffectType, VACUUM_ATMOSEFFECT};

/// When a cell gets removed, ie deconstruct event on floor or wall tile, update atmospherics.

pub(crate) fn remove_cell_atmos_event(
    mut deconstruct_cell_events: EventReader<RemoveCell>,
    gridmap_main: Res<Gridmap>,
    mut atmospherics_resource: ResMut<AtmosphericsResource>,
) {
    for event in deconstruct_cell_events.iter() {
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
            match gridmap_main.get_cell(upper_id) {
                Some(_) => {}
                None => {
                    atmospherics
                        .effects
                        .insert(EffectType::Floorless, VACUUM_ATMOSEFFECT);
                }
            }
        }
    }
}
