use bevy::prelude::Res;
use bevy::prelude::{Commands, EventReader};

use crate::build::{EnableMainMenu, MainMenuState};

/// System that toggles the visiblity of the main menu based on an event.
#[cfg(feature = "client")]
pub(crate) fn hide_main_menu(
    mut _enable_events: EventReader<EnableMainMenu>,
    state: Res<MainMenuState>,
    mut _commands: Commands,
) {
    if !state.enabled {
        return;
    }
}
