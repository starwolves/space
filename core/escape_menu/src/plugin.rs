use bevy::prelude::{App, FixedUpdate, IntoSystemConfigs, Plugin, PostStartup, Startup, Update};
use graphics::settings::{set_vsync, SettingsSet};
use hud::{inventory::build::open_hud, mouse::grab_mouse_hud_expand};
use resources::{is_server::is_server, sets::MainSet};
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
                FixedUpdate,
                (
                    build_graphics_section.after(SettingsSet::Apply),
                    build_controls_section,
                ),
            )
            .add_systems(
                Update,
                (
                    toggle_general_menu_section
                        .after(general_section_button_pressed)
                        .after(graphics_section_button_pressed)
                        .after(controls_section_button_pressed),
                    toggle_graphics_menu_section
                        .after(general_section_button_pressed)
                        .after(graphics_section_button_pressed)
                        .after(controls_section_button_pressed),
                    toggle_controls_menu_section
                        .after(general_section_button_pressed)
                        .after(graphics_section_button_pressed)
                        .after(controls_section_button_pressed),
                ),
            )
            .add_systems(
                FixedUpdate,
                (
                    toggle_escape_menu
                        .before(grab_mouse_hud_expand)
                        .before(open_hud)
                        .before(toggle_general_menu_section)
                        .before(toggle_graphics_menu_section)
                        .before(toggle_controls_menu_section),
                    esc_button_menu.before(toggle_escape_menu),
                    exit_button_pressed,
                    general_section_button_pressed.before(toggle_general_menu_section),
                    graphics_section_button_pressed,
                    controls_section_button_pressed,
                    appply_resolution.before(SettingsSet::Apply),
                    apply_window_mode.before(SettingsSet::Apply),
                    apply_vsync.before(SettingsSet::Apply),
                    apply_fxaa.before(SettingsSet::Apply),
                    apply_msaa.before(SettingsSet::Apply),
                )
                    .in_set(MainSet::Update),
            )
            .add_event::<ToggleEscapeMenu>()
            .add_event::<ToggleGeneralSection>()
            .add_event::<ToggleGraphicsSection>()
            .add_event::<ToggleControlsSection>();
        }
    }
}
