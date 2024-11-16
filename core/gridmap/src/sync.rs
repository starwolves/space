use bevy::prelude::{Commands, EventWriter, Local, Res};
use networking::stamp::TickRateStamp;
use resources::correction::{IsCorrecting, StartCorrection};

use crate::grid::{AddTile, Gridmap, RemoveTile};

/// Given our start correction point has the correct gridmap loaded all we need to do from the ticks onwards is do the synced add and remove tiles.
pub(crate) fn correction_gridmap_sync(
    stamp: Res<TickRateStamp>,
    gridmap: Res<Gridmap>,
    correcting: Res<IsCorrecting>,
    mut add_tile: EventWriter<AddTile>,
    mut remove_tile: EventWriter<RemoveTile>,
    start: Res<StartCorrection>,
    mut commands: Commands,
    mut gridmap_tick: Local<u32>,
) {
    if !correcting.0 {
        return;
    }
    // First correction tick
    if stamp.tick == start.start_tick {
        // Figure out the stamp the gridmap is currently at.
        // Send all gridmap.updates from this gridmap tick to start.start_tick
        gridmap.batch_updates(
            *gridmap_tick,
            start.start_tick,
            &mut add_tile,
            &mut remove_tile,
            &mut commands,
        );
        *gridmap_tick = stamp.tick;
    } else {
        match gridmap.updates.get(&stamp.tick) {
            Some(updates) => {
                for (target, update) in updates.iter() {
                    match &update.cell {
                        crate::grid::GridmapUpdate::Added(new_cell) => {
                            add_tile.send(AddTile {
                                id: target.target.id,
                                tile_type: new_cell.tile_type,
                                orientation: new_cell.orientation,
                                face: target.target.face.clone(),
                                group_instance_id_option: None,
                                entity: commands.spawn(()).id(),
                                default_map_spawn: false,
                                is_detail: target.is_detail,
                                stamp: stamp.tick,
                            });
                        }
                        crate::grid::GridmapUpdate::Removed => {
                            remove_tile.send(RemoveTile {
                                cell: target.clone(),
                                stamp: stamp.tick,
                            });
                        }
                    }
                }

                *gridmap_tick = stamp.tick;
            }
            None => {}
        }
    }
}
