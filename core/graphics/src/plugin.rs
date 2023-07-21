use bevy::prelude::{App, Plugin, Startup, Update};
use resources::is_server::is_server;

use crate::{
    settings::{
        init_light, set_fxaa, set_msaa, set_resolution, set_vsync, set_window_mode,
        settings_to_ron, setup_graphics_settings, GraphicsSettings, SetFxaa, SetMsaa,
        SetResolution, SetVsync, SetWindowMode,
    },
    skybox::preload_skybox,
    tonemapping::PerMethodSettings,
};

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        if !is_server() {
            app.add_systems(
                Update,
                (
                    settings_to_ron,
                    set_fxaa,
                    set_msaa,
                    set_resolution,
                    set_vsync,
                    set_window_mode,
                ),
            )
            .add_systems(
                Startup,
                (preload_skybox, init_light, setup_graphics_settings),
            )
            .add_event::<SetResolution>()
            .init_resource::<GraphicsSettings>()
            .add_event::<SetWindowMode>()
            .add_event::<SetVsync>()
            .add_event::<SetFxaa>()
            .add_event::<SetMsaa>()
            .init_resource::<PerMethodSettings>();
            /*.add_plugins(AtmospherePlugin)
            .add_systems(Startup, add_atmosphere)*/
            //.add_systems(Update, toggle_tonemapping_method);
        }
    }
}
