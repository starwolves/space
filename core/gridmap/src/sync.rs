use bevy::prelude::Res;
use networking::stamp::TickRateStamp;

use crate::grid::Gridmap;

pub(crate) fn sync_gridmap(stamp: Res<TickRateStamp>, gridmap: Res<Gridmap>) {}
