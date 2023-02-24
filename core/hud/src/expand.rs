use bevy::prelude::{EventReader, ResMut};
use cameras::controllers::fps::CameraMouseInputEnabled;
use resources::hud::HudState;

/// Event to expand the hud.
pub struct ExpandInventoryHud {
    pub expand: bool,
}

pub(crate) fn expand_hud(
    mut events: EventReader<ExpandInventoryHud>,
    mut state: ResMut<HudState>,
    mut mouse_enabled: ResMut<CameraMouseInputEnabled>,
) {
    for event in events.iter() {
        state.expanded = event.expand;
        mouse_enabled.enabled = !event.expand;
    }
}
