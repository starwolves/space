use bevy::prelude::{EventReader, ResMut};

use crate::hud::HudState;

/// Event to expand the hud.
pub struct ExpandHud {
    pub expand: bool,
}

pub(crate) fn expand_hud(mut events: EventReader<ExpandHud>, mut state: ResMut<HudState>) {
    for event in events.iter() {
        state.expanded = event.expand;
    }
}
