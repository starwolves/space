use bevy::prelude::{App, IntoSystemConfigs, Plugin, Startup};
use resources::{modes::is_server_mode, ordering::Update};

use crate::{
    settings::{
        init_light, set_fxaa, set_msaa, set_rcas, set_resolution, set_vsync, set_window_mode,
        settings_to_ron, setup_graphics_settings, GraphicsSettings, SetFxaa, SetMsaa, SetRCAS,
        SetResolution, SetVsync, SetWindowMode, SettingsSet,
    },
    skybox::preload_skybox,
    tonemapping::PerMethodSettings,
};

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        if !is_server_mode(app) {
            app.add_systems(
                Update,
                (
                    settings_to_ron,
                    set_fxaa,
                    set_msaa,
                    set_resolution,
                    set_vsync,
                    set_rcas,
                    set_window_mode,
                )
                    .after(SettingsSet::Apply),
            )
            .add_systems(
                Startup,
                (
                    preload_skybox,
                    init_light,
                    setup_graphics_settings.before(SettingsSet::Apply),
                ),
            )
            .add_event::<SetResolution>()
            .init_resource::<GraphicsSettings>()
            .add_event::<SetWindowMode>()
            .add_event::<SetVsync>()
            .add_event::<SetFxaa>()
            .add_event::<SetRCAS>()
            .add_event::<SetMsaa>()
            .init_resource::<PerMethodSettings>();
            /*.add_plugins(AtmospherePlugin)
            .add_systems(Startup, add_atmosphere)*/
            //.add_systems(FixedUpdate, toggle_tonemapping_method);
        }
    }
}
