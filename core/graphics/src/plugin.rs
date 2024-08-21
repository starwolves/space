use bevy::prelude::{App, IntoSystemConfigs, Plugin, Startup};
use resources::{modes::is_server_mode, ordering::Update};

use crate::{
    settings::{
        forward_performance_settings, init_light, set_ambient_lighting, set_fxaa, set_msaa,
        set_rcas, set_resolution, set_shadow_filter, set_shadows, set_shadows_cascading,
        set_shadows_resolution, set_ssao, set_ssr, set_sync_correction, set_vsync, set_window_mode,
        settings_to_ron, PerformanceSettings, SetAmbientLighting, SetFxaa, SetMsaa, SetRCAS,
        SetResolution, SetSSAO, SetSSR, SetShadowFilter, SetShadows, SetShadowsCascading,
        SetShadowsResolution, SetSyncCorrection, SetVsync, SetWindowMode, SettingsSet,
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
                    set_ambient_lighting,
                    set_shadows.before(set_ambient_lighting),
                    set_shadows_resolution,
                    set_shadows_cascading,
                    set_ssao,
                    set_ssr,
                    set_shadow_filter,
                    set_window_mode,
                    set_sync_correction,
                )
                    .after(SettingsSet::Apply),
            )
            .add_systems(
                Startup,
                (
                    preload_skybox,
                    init_light.after(set_shadows),
                    forward_performance_settings.before(SettingsSet::Apply),
                ),
            )
            .add_event::<SetResolution>()
            .init_resource::<PerformanceSettings>()
            .add_event::<SetWindowMode>()
            .add_event::<SetVsync>()
            .add_event::<SetFxaa>()
            .add_event::<SetSyncCorrection>()
            .add_event::<SetRCAS>()
            .add_event::<SetAmbientLighting>()
            .add_event::<SetShadows>()
            .add_event::<SetShadowsResolution>()
            .add_event::<SetShadowsCascading>()
            .add_event::<SetMsaa>()
            .add_event::<SetSSAO>()
            .add_event::<SetShadowFilter>()
            .add_event::<SetSSR>()
            .init_resource::<PerMethodSettings>();
            /*.add_plugins(AtmospherePlugin)
            .add_systems(Startup, add_atmosphere)*/
            //.add_systems(FixedUpdate, toggle_tonemapping_method);
        }
    }
}
