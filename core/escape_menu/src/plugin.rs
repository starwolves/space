use bevy::prelude::{App, IntoSystemConfig, Plugin, StartupSet};
use resources::is_server::is_server;

use crate::{
    build::{build_controls_section, build_escape_menu, build_graphics_section},
    events::{
        controls_section_button_pressed, esc_button_menu, exit_button_pressed,
        general_section_button_pressed, graphics_section_button_pressed, register_input,
        toggle_controls_menu_section, toggle_escape_menu, toggle_general_menu_section,
        toggle_graphics_menu_section, ToggleControlsSection, ToggleEscapeMenu,
        ToggleGeneralSection, ToggleGraphicsSection,
    },
};

pub struct EscapeMenuPlugin;

impl Plugin for EscapeMenuPlugin {
    fn build(&self, app: &mut App) {
        if !is_server() {
            app.add_startup_system(build_escape_menu)
                .add_startup_system(build_controls_section.in_base_set(StartupSet::PostStartup))
                .add_startup_system(build_graphics_section.in_base_set(StartupSet::PostStartup))
                .add_system(toggle_escape_menu)
                .add_event::<ToggleEscapeMenu>()
                .add_system(esc_button_menu)
                .add_system(toggle_general_menu_section)
                .add_system(toggle_graphics_menu_section)
                .add_system(toggle_controls_menu_section)
                .add_event::<ToggleGeneralSection>()
                .add_event::<ToggleGraphicsSection>()
                .add_event::<ToggleControlsSection>()
                .add_system(exit_button_pressed)
                .add_system(general_section_button_pressed)
                .add_system(graphics_section_button_pressed)
                .add_system(controls_section_button_pressed)
                .add_startup_system(register_input);
        }
    }
}
