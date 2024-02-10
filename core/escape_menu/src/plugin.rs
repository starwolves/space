use bevy::prelude::{App, IntoSystemConfigs, Plugin, PostStartup, Startup, Update as BevyUpdate};
use graphics::settings::SettingsSet;
use hud::{inventory::build::OpenHudSet, mouse::grab_mouse_hud_expand};
use resources::{modes::is_server_mode, ordering::Update};
use ui::fonts::init_fonts;

use crate::{
    build::{build_controls_section, build_escape_menu, build_graphics_section},
    events::{
        apply_ambient_lighting, apply_fxaa, apply_msaa, apply_rcas, apply_shadows_setting,
        apply_ssao_setting, apply_syncronous_correction_setting, apply_vsync, apply_window_mode,
        appply_resolution, controls_section_button_pressed, esc_button_menu, exit_button_pressed,
        general_section_button_pressed, graphics_section_button_pressed, register_input,
        toggle_controls_menu_section, toggle_escape_menu, toggle_general_menu_section,
        toggle_graphics_menu_section, ToggleControlsSection, ToggleEscapeMenu,
        ToggleGeneralSection, ToggleGraphicsSection,
    },
};

pub struct EscapeMenuPlugin;

impl Plugin for EscapeMenuPlugin {
    fn build(&self, app: &mut App) {
        if !is_server_mode(app) {
            app.add_systems(
                Startup,
                (build_escape_menu.after(init_fonts), register_input),
            )
            .add_systems(
                PostStartup,
                (
                    build_controls_section,
                    build_graphics_section.after(SettingsSet::Apply),
                ),
            )
            .add_systems(
                BevyUpdate,
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
                Update,
                (
                    toggle_escape_menu
                        .before(OpenHudSet::Process)
                        .before(grab_mouse_hud_expand)
                        .before(toggle_general_menu_section)
                        .before(toggle_graphics_menu_section)
                        .before(toggle_controls_menu_section),
                    esc_button_menu.before(toggle_escape_menu),
                    exit_button_pressed,
                    general_section_button_pressed.before(toggle_general_menu_section),
                    graphics_section_button_pressed,
                    controls_section_button_pressed,
                    (
                        appply_resolution,
                        apply_window_mode,
                        apply_vsync,
                        apply_rcas,
                        apply_fxaa,
                        apply_shadows_setting,
                        apply_ssao_setting,
                        apply_ambient_lighting,
                        apply_msaa,
                        apply_syncronous_correction_setting,
                    )
                        .before(SettingsSet::Apply),
                ),
            )
            .add_event::<ToggleEscapeMenu>()
            .add_event::<ToggleGeneralSection>()
            .add_event::<ToggleGraphicsSection>()
            .add_event::<ToggleControlsSection>();
        }
    }
}
