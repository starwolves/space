use std::{
    f32::consts::PI,
    fs::{self, create_dir_all},
    path::Path,
};

use bevy::{
    core_pipeline::fxaa::{Fxaa, Sensitivity},
    pbr::light_consts::lux::DIRECT_SUNLIGHT,
    prelude::{
        Commands, DetectChanges, DirectionalLight, DirectionalLightBundle, Event, EventReader,
        EventWriter, Msaa, Quat, Query, Res, ResMut, Resource, SystemSet, Transform, With,
    },
    window::{PresentMode, PrimaryWindow, Window, WindowMode, WindowResolution},
};
use bevy::{
    core_pipeline::{
        contrast_adaptive_sharpening::ContrastAdaptiveSharpeningSettings, core_3d::Camera3d,
    },
    ecs::entity::Entity,
    log::warn,
    pbr::{
        AmbientLight, PointLight, ScreenSpaceAmbientOcclusionQualityLevel,
        ScreenSpaceAmbientOcclusionSettings,
    },
};

use num_derive::FromPrimitive;
use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};

pub(crate) fn default_ambient_light(enabled: bool) -> AmbientLight {
    if enabled {
        AmbientLight::default()
    } else {
        AmbientLight {
            brightness: 0.,
            ..Default::default()
        }
    }
}

pub(crate) fn init_light(mut commands: Commands, settings: Res<PerformanceSettings>) {
    commands.insert_resource(default_ambient_light(settings.ambient_lighting));

    let directional_shadows;
    match settings.shadows {
        Shadows::Off => {
            directional_shadows = false;
        }
        Shadows::Medium => {
            directional_shadows = true;
        }
        Shadows::High => {
            directional_shadows = true;
        }
    }
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: directional_shadows,
            illuminance: DIRECT_SUNLIGHT * 100.,
            ..Default::default()
        },
        transform: Transform {
            rotation: Quat::from_rotation_y(-PI * 1.).mul_quat(Quat::from_rotation_x(-PI * 0.1)),
            ..Default::default()
        },
        ..Default::default()
    });
}

#[derive(Resource, Serialize, Deserialize, Clone)]
pub struct PerformanceSettings {
    pub resolution: (u32, u32),
    pub window_mode: SFWindowMode,
    pub vsync: bool,
    pub fxaa: Option<SFFxaa>,
    pub msaa: SFMsaa,
    pub rcas: bool,
    pub shadows: Shadows,
    pub synchronous_correction: bool,
    pub ambient_lighting: bool,
    pub ssao: SSAO,
}
#[derive(Default, Clone, Serialize, Deserialize, FromPrimitive, Debug)]
pub enum SSAO {
    Off = 0,
    Low = 1,
    #[default]
    Medium = 2,
    High = 3,
    Ultra = 4,
}
impl SSAO {
    pub fn to_quality(&self) -> ScreenSpaceAmbientOcclusionQualityLevel {
        match self {
            SSAO::Off => ScreenSpaceAmbientOcclusionQualityLevel::Low,
            SSAO::Low => ScreenSpaceAmbientOcclusionQualityLevel::Low,
            SSAO::Medium => ScreenSpaceAmbientOcclusionQualityLevel::Medium,
            SSAO::High => ScreenSpaceAmbientOcclusionQualityLevel::High,
            SSAO::Ultra => ScreenSpaceAmbientOcclusionQualityLevel::Ultra,
        }
    }
}
#[derive(Default, Clone, Serialize, Deserialize, FromPrimitive, Debug)]
pub enum Shadows {
    Off = 0,
    #[default]
    Medium = 1,
    High = 2,
}
#[derive(Default, Clone, Serialize, Deserialize, FromPrimitive, Debug)]
pub enum SFMsaa {
    #[default]
    Off = 0,
    Sample2 = 1,
    Sample4 = 2,
    Sample8 = 3,
}
impl SFMsaa {
    pub fn to_msaa(&self) -> Msaa {
        match self {
            SFMsaa::Off => Msaa::Off,
            SFMsaa::Sample2 => Msaa::Sample2,
            SFMsaa::Sample4 => Msaa::Sample4,
            SFMsaa::Sample8 => Msaa::Sample8,
        }
    }
    pub fn is_enabled(&self) -> bool {
        match self {
            SFMsaa::Off => false,
            _ => true,
        }
    }
}
#[derive(Serialize, Deserialize, Default, Clone, FromPrimitive, Debug)]
pub enum SFFxaa {
    Low = 0,
    Medium = 1,
    #[default]
    High = 2,
}
impl SFFxaa {
    pub fn to_sensitivity(&self) -> Sensitivity {
        match self {
            SFFxaa::Low => Sensitivity::Low,
            SFFxaa::Medium => Sensitivity::Medium,
            SFFxaa::High => Sensitivity::High,
        }
    }
}
#[derive(Serialize, Deserialize, Default, Clone, FromPrimitive, Debug)]
pub enum SFWindowMode {
    #[default]
    Windowed = 0,
    BorderlessFullscreen = 1,
    SizedFullscreen = 2,
    Fullscreen = 3,
}
impl SFWindowMode {
    pub fn to_window_mode(&self) -> WindowMode {
        match self {
            SFWindowMode::Windowed => WindowMode::Windowed,
            SFWindowMode::BorderlessFullscreen => WindowMode::BorderlessFullscreen,
            SFWindowMode::SizedFullscreen => WindowMode::SizedFullscreen,
            SFWindowMode::Fullscreen => WindowMode::Fullscreen,
        }
    }
}

impl Default for PerformanceSettings {
    fn default() -> Self {
        let default_res = WindowResolution::default();
        Self {
            resolution: (default_res.physical_width(), default_res.physical_height()),
            window_mode: SFWindowMode::default(),
            vsync: false,
            fxaa: Some(SFFxaa::default()),
            msaa: SFMsaa::Off,
            rcas: true,
            synchronous_correction: false,
            shadows: Shadows::default(),
            ambient_lighting: false,
            ssao: SSAO::default(),
        }
    }
}

pub fn get_settings() -> PerformanceSettings {
    let path = Path::new("data").join("settings").join("settings.ron");
    let settings_folder = Path::new("data").join("settings");

    let mut generate_new_config = !path.exists();
    let settings;
    if path.exists() {
        let settings_ron = fs::read_to_string(path.clone()).unwrap();
        match ron::from_str(&settings_ron) {
            Ok(s) => settings = s,
            Err(_) => {
                generate_new_config = true;
                settings = PerformanceSettings::default();
            }
        }
    } else {
        settings = PerformanceSettings::default();
    }

    if generate_new_config {
        if !settings_folder.exists() {
            create_dir_all(settings_folder).unwrap();
        }
        let settings_ron = ron::ser::to_string_pretty(&settings, PrettyConfig::default()).unwrap();
        fs::write(path, settings_ron).unwrap();
    }
    settings
}

pub(crate) fn forward_performance_settings(
    settings: Res<PerformanceSettings>,
    mut res_events: EventWriter<SetResolution>,
    mut vsync_events: EventWriter<SetVsync>,
    mut w_mode_events: EventWriter<SetWindowMode>,
    mut fxaa_events: EventWriter<SetFxaa>,
    mut msaa_events: EventWriter<SetMsaa>,
    //mut rcas: EventWriter<SetRCAS>,
    mut shadows: EventWriter<SetShadows>,
    mut ambient: EventWriter<SetAmbientLighting>,
    //mut ssao: EventWriter<SetSSAO>,
    mut sync: EventWriter<SetSyncCorrection>,
) {
    res_events.send(SetResolution {
        resolution: (settings.resolution.0, settings.resolution.1),
    });
    vsync_events.send(SetVsync {
        enabled: settings.vsync,
    });
    w_mode_events.send(SetWindowMode {
        window_mode: settings.window_mode.clone(),
    });
    fxaa_events.send(SetFxaa {
        mode: settings.fxaa.clone(),
    });
    msaa_events.send(SetMsaa {
        mode: settings.msaa.clone(),
    });
    shadows.send(SetShadows {
        mode: settings.shadows.clone(),
    });
    ambient.send(SetAmbientLighting {
        enabled: settings.ambient_lighting,
    });
    sync.send(SetSyncCorrection {
        enabled: settings.synchronous_correction,
    });
}
#[derive(Event)]
pub struct SetResolution {
    pub resolution: (u32, u32),
}

pub(crate) fn set_resolution(
    mut events: EventReader<SetResolution>,
    mut primary_query: Query<&mut Window, With<PrimaryWindow>>,
    mut settings: ResMut<PerformanceSettings>,
) {
    for event in events.read() {
        let mut primary = primary_query.get_single_mut().unwrap();
        primary
            .resolution
            .set(event.resolution.0 as f32, event.resolution.1 as f32);
        settings.resolution = (event.resolution.0, event.resolution.1);
    }
}
#[derive(Event)]
pub struct SetVsync {
    pub enabled: bool,
}
#[derive(Event)]
pub struct SetRCAS {
    pub enabled: bool,
}
#[derive(Event)]
pub struct SetAmbientLighting {
    pub enabled: bool,
}
#[derive(Event)]
pub struct SetShadows {
    pub mode: Shadows,
}
#[derive(Event)]
pub struct SetSSAO {
    pub mode: SSAO,
}

#[derive(Event)]
pub struct SetSyncCorrection {
    pub enabled: bool,
}
pub fn set_sync_correction(
    mut settings: ResMut<PerformanceSettings>,
    mut events: EventReader<SetSyncCorrection>,
) {
    for e in events.read() {
        settings.synchronous_correction = e.enabled;
    }
}
pub fn set_vsync(
    mut events: EventReader<SetVsync>,
    mut primary_query: Query<&mut Window, With<PrimaryWindow>>,
    mut settings: ResMut<PerformanceSettings>,
) {
    for event in events.read() {
        let mut primary = primary_query.get_single_mut().unwrap();
        if event.enabled {
            primary.present_mode = PresentMode::AutoVsync;
        } else {
            primary.present_mode = PresentMode::AutoNoVsync;
        }
        settings.vsync = event.enabled;
    }
}
pub fn set_rcas(
    mut events: EventReader<SetRCAS>,
    mut primary_query: Query<&mut ContrastAdaptiveSharpeningSettings>,
    mut settings: ResMut<PerformanceSettings>,
) {
    for event in events.read() {
        match primary_query.get_single_mut() {
            Ok(mut settings) => {
                settings.enabled = event.enabled;
            }
            Err(_) => {
                warn!("No camera for rcas settings found.");
            }
        }
        settings.rcas = event.enabled;
    }
}
pub fn set_ambient_lighting(
    mut events: EventReader<SetAmbientLighting>,
    // mut settings: ResMut<PerformanceSettings>,
    mut ambient: ResMut<AmbientLight>,
) {
    for event in events.read() {
        //settings.ambient_lighting = event.enabled;
        if event.enabled {
            ambient.brightness = AmbientLight::default().brightness;
        } else {
            ambient.brightness = 0.
        }
    }
}
pub fn set_ssao(
    mut events: EventReader<SetSSAO>,
    mut commands: Commands,
    mut settings: ResMut<PerformanceSettings>,
    mut query: Query<(Entity, &mut ScreenSpaceAmbientOcclusionSettings)>,
    camera_query: Query<Entity, With<Camera3d>>,
) {
    for event in events.read() {
        settings.ssao = event.mode.clone();
        let quality_level = event.mode.to_quality();
        let mut turn_off = false;

        match event.mode {
            SSAO::Off => {
                turn_off = true;
            }
            _ => (),
        }
        match query.get_single_mut() {
            Ok((entity, mut settings)) => {
                if turn_off {
                    commands
                        .entity(entity)
                        .remove::<ScreenSpaceAmbientOcclusionSettings>();
                } else {
                    *settings = ScreenSpaceAmbientOcclusionSettings { quality_level };
                }
            }
            Err(_) => {
                let camera_entity = camera_query.single();
                commands
                    .entity(camera_entity)
                    .insert(ScreenSpaceAmbientOcclusionSettings { quality_level });
            }
        }
    }
}

pub fn set_shadows(
    mut events: EventReader<SetShadows>,
    mut directional_lights: Query<&mut DirectionalLight>,
    mut point_lights: Query<&mut PointLight>,
    mut settings: ResMut<PerformanceSettings>,
    mut set_ambience: EventWriter<SetAmbientLighting>,
) {
    for event in events.read() {
        settings.shadows = event.mode.clone();

        let directional_lights_enabled;
        let point_lights_enabled;
        let ambience_enabled;
        match &settings.shadows {
            Shadows::Off => {
                directional_lights_enabled = false;
                point_lights_enabled = false;
                ambience_enabled = false;
            }
            Shadows::Medium => {
                directional_lights_enabled = true;
                point_lights_enabled = false;
                ambience_enabled = true;
            }
            Shadows::High => {
                directional_lights_enabled = true;
                point_lights_enabled = true;
                ambience_enabled = true;
            }
        }
        set_ambience.send(SetAmbientLighting {
            enabled: ambience_enabled,
        });

        for mut p in point_lights.iter_mut() {
            p.shadows_enabled = point_lights_enabled;
            if ambience_enabled {
                p.intensity = 1200.
            } else {
                p.intensity = 1800.
            }
        }
        for mut d in directional_lights.iter_mut() {
            d.shadows_enabled = directional_lights_enabled;
        }
    }
}
/// Label for systems ordering.
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum SettingsSet {
    Apply,
}

#[derive(Event)]
pub struct SetWindowMode {
    pub window_mode: SFWindowMode,
}
pub(crate) fn set_window_mode(
    mut events: EventReader<SetWindowMode>,
    mut primary_query: Query<&mut Window, With<PrimaryWindow>>,
    mut settings: ResMut<PerformanceSettings>,
) {
    for event in events.read() {
        let mut primary = primary_query.get_single_mut().unwrap();

        primary.mode = event.window_mode.to_window_mode();

        settings.window_mode = event.window_mode.clone();
    }
}
#[derive(Event)]
pub struct SetFxaa {
    pub mode: Option<SFFxaa>,
}
pub(crate) fn set_fxaa(
    mut events: EventReader<SetFxaa>,
    mut settings: ResMut<PerformanceSettings>,
    mut query: Query<&mut Fxaa>,
) {
    for event in events.read() {
        settings.fxaa = event.mode.clone();

        for mut fxaa in &mut query {
            fxaa.enabled = settings.fxaa.is_some();
            match &settings.fxaa {
                Some(t) => {
                    fxaa.edge_threshold = t.to_sensitivity();
                }
                None => {}
            }
        }
    }
}
#[derive(Event)]
pub struct SetMsaa {
    pub mode: SFMsaa,
}
pub(crate) fn set_msaa(
    mut events: EventReader<SetMsaa>,
    mut settings: ResMut<PerformanceSettings>,
    mut msaa: ResMut<Msaa>,
) {
    for event in events.read() {
        settings.msaa = event.mode.clone();
        *msaa = settings.msaa.to_msaa();
    }
}

pub(crate) fn settings_to_ron(settings: Res<PerformanceSettings>) {
    if settings.is_changed() {
        let path = Path::new("data").join("settings").join("settings.ron");
        let settings_ron = ron::ser::to_string_pretty(&*settings, PrettyConfig::default()).unwrap();
        match fs::write(path, settings_ron) {
            Ok(_) => {}
            Err(rr) => {
                warn!("Failed to write settings.ron: {:?}", rr);
            }
        };
    }
}
