use bevy::prelude::{App, Plugin};
use resources::is_server::is_server;

use crate::{
    settings::{
        set_fxaa, set_msaa, set_resolution, set_vsync, set_window_mode, settings_to_ron,
        setup_graphics_settings, GraphicsSettings, SetFxaa, SetMsaa, SetResolution, SetVsync,
        SetWindowMode,
    },
    tonemapping::PerMethodSettings,
};

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        if !is_server() {
            app.add_system(set_resolution)
                .add_event::<SetResolution>()
                .init_resource::<GraphicsSettings>()
                .add_startup_system(setup_graphics_settings)
                .add_system(set_vsync)
                .add_event::<SetWindowMode>()
                .add_event::<SetVsync>()
                .add_system(set_window_mode)
                .add_event::<SetFxaa>()
                .add_event::<SetMsaa>()
                .add_system(set_msaa)
                .add_system(set_fxaa)
                .add_system(settings_to_ron)
                .init_resource::<PerMethodSettings>();
            //.add_system(toggle_tonemapping_method);
        }
    }
}
