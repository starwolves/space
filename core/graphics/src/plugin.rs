use bevy::prelude::{App, FixedUpdate, IntoSystemConfigs, Plugin, Startup};
use resources::{modes::is_server, sets::MainSet};

use crate::{
    settings::{
        init_light, set_fxaa, set_msaa, set_resolution, set_vsync, set_window_mode,
        settings_to_ron, setup_graphics_settings, GraphicsSettings, SetFxaa, SetMsaa,
        SetResolution, SetVsync, SetWindowMode, SettingsSet,
    },
    skybox::preload_skybox,
    tonemapping::PerMethodSettings,
};

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        if !is_server() {
            app.add_systems(
                FixedUpdate,
                (
                    settings_to_ron.after(SettingsSet::Apply),
                    set_fxaa.in_set(SettingsSet::Apply),
                    set_msaa.in_set(SettingsSet::Apply),
                    set_resolution.in_set(SettingsSet::Apply),
                    set_vsync.in_set(SettingsSet::Apply),
                    set_window_mode.in_set(SettingsSet::Apply),
                    setup_graphics_settings.before(SettingsSet::Apply),
                )
                    .in_set(MainSet::Update),
            )
            .add_systems(Startup, (preload_skybox, init_light))
            .add_event::<SetResolution>()
            .init_resource::<GraphicsSettings>()
            .add_event::<SetWindowMode>()
            .add_event::<SetVsync>()
            .add_event::<SetFxaa>()
            .add_event::<SetMsaa>()
            .init_resource::<PerMethodSettings>();
            /*.add_plugins(AtmospherePlugin)
            .add_systems(Startup, add_atmosphere)*/
            //.add_systems(FixedUpdate, toggle_tonemapping_method);
        }
    }
}
