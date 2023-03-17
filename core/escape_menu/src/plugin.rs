use bevy::prelude::{App, Plugin};
use resources::is_server::is_server;

use crate::{
    build::build_escape_menu,
    toggle::{
        esc_button_menu, toggle_escape_menu, toggle_general_menu_section,
        toggle_graphics_menu_section, ToggleEscapeMenu, ToggleGeneralSection,
        ToggleGraphicsSection,
    },
};

pub struct EscapeMenuPlugin;

impl Plugin for EscapeMenuPlugin {
    fn build(&self, app: &mut App) {
        if !is_server() {
            app.add_startup_system(build_escape_menu)
                .add_system(toggle_escape_menu)
                .add_event::<ToggleEscapeMenu>()
                .add_system(esc_button_menu)
                .add_system(toggle_general_menu_section)
                .add_system(toggle_graphics_menu_section)
                .add_event::<ToggleGeneralSection>()
                .add_event::<ToggleGraphicsSection>();
        }
    }
}
