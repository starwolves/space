use bevy::{
    prelude::{EventReader, EventWriter, Input, KeyCode, Query, Res, ResMut},
    ui::{Display, Style},
};
use hud::inventory::build::OpenHud;
use resources::hud::HudState;

use crate::build::EscapeMenuState;

pub struct ToggleEscapeMenu {
    pub enabled: bool,
}

pub(crate) fn esc_button_menu(
    keys: Res<Input<KeyCode>>,
    mut events: EventWriter<ToggleEscapeMenu>,
    state: Res<EscapeMenuState>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        events.send(ToggleEscapeMenu {
            enabled: !state.visible,
        });
    }
}

pub(crate) fn toggle_escape_menu(
    mut style_query: Query<&mut Style>,
    mut state: ResMut<EscapeMenuState>,
    mut events: EventReader<ToggleEscapeMenu>,
    hud_state: Res<HudState>,
    mut hud: EventWriter<OpenHud>,
    mut toggle_general: EventWriter<ToggleGeneralSection>,
) {
    for toggle in events.iter() {
        state.visible = toggle.enabled;

        hud.send(OpenHud {
            open: state.visible,
        });

        let mut esc_root_style = style_query.get_mut(state.root).unwrap();
        if state.visible {
            esc_root_style.display = Display::Flex;
        } else {
            esc_root_style.display = Display::None;
        }
        let mut hud_root_style = style_query.get_mut(hud_state.root_entity).unwrap();
        if !state.visible {
            hud_root_style.display = Display::Flex;
        } else {
            hud_root_style.display = Display::None;
        }

        if state.visible {
            toggle_general.send(ToggleGeneralSection { enabled: true });
        }
    }
}

pub struct ToggleGeneralSection {
    pub enabled: bool,
}
pub struct ToggleGraphicsSection {
    pub enabled: bool,
}

pub(crate) fn toggle_general_menu_section(mut events: EventReader<ToggleGeneralSection>) {
    for toggle in events.iter() {}
}

pub(crate) fn toggle_graphics_menu_section(mut events: EventReader<ToggleGraphicsSection>) {
    for toggle in events.iter() {}
}
