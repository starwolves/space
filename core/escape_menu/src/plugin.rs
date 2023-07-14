use bevy::prelude::{App, IntoSystemConfigs, Plugin, PostStartup, Startup, Update};
use resources::is_server::is_server;
use ui::fonts::init_fonts;

use crate::{
    build::{build_controls_section, build_escape_menu, build_graphics_section},
    events::{
        apply_fxaa, apply_msaa, apply_vsync, apply_window_mode, appply_resolution,
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
            app.add_systems(
                Startup,
                (build_escape_menu.after(init_fonts), register_input),
            )
            .add_systems(
                PostStartup,
                (build_graphics_section, build_controls_section),
            )
            .add_systems(
                Update,
                (
                    toggle_escape_menu,
                    esc_button_menu,
                    toggle_general_menu_section,
                    toggle_graphics_menu_section,
                    toggle_controls_menu_section,
                    exit_button_pressed,
                    general_section_button_pressed,
                    graphics_section_button_pressed,
                    controls_section_button_pressed,
                    appply_resolution,
                    apply_window_mode,
                    apply_vsync,
                    apply_fxaa,
                    apply_msaa,
                ),
            )
            .add_event::<ToggleEscapeMenu>()
            .add_event::<ToggleGeneralSection>()
            .add_event::<ToggleGraphicsSection>()
            .add_event::<ToggleControlsSection>();
        }
    }
}
