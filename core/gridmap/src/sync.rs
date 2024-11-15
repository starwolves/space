use bevy::prelude::{EventWriter, Res};
use networking::stamp::TickRateStamp;
use resources::correction::{IsCorrecting, StartCorrection};

use crate::grid::{AddTile, Gridmap, RemoveTile};

/// Given our start correction point has the correct gridmap loaded all we need to do from the ticks onwards is do the synced add and remove tiles.
/// BUT what if we just simulated to tick 55, now we have a new calc of 53 to 56, the first sim tick needs to go back in time and sync the gridmap.
pub(crate) fn correction_collision(
    stamp: Res<TickRateStamp>,
    gridmap: Res<Gridmap>,
    correcting: Res<IsCorrecting>,
    mut add_tile: EventWriter<AddTile>,
    mut remove_tile: EventWriter<RemoveTile>,
    start: Res<StartCorrection>,
) {
    let cache_tick = stamp.tick;
    if !correcting.0 {
        return;
    }

    // First correction tick
    if stamp.tick == start.start_tick {
        // Figure out the stamp the gridmap is currently at.
        // Send all gridmap.updates from this gridmap tick to start.start_tick
    }
}
