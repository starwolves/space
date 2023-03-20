use std::{fs, path::Path};

use bevy::{
    core_pipeline::fxaa::{Fxaa, Sensitivity},
    prelude::{EventReader, EventWriter, Msaa, Query, ResMut, Resource, With},
    window::{PresentMode, PrimaryWindow, Window, WindowMode, WindowResolution},
};
use ron::ser::PrettyConfig;
use serde::{Deserialize, Serialize};

#[derive(Resource, Serialize, Deserialize)]
pub struct GraphicsSettings {
    pub resolution: (u32, u32),
    pub window_mode: SFWindowMode,
    pub vsync: bool,
    pub fxaa: Option<SFFxaa>,
    pub msaa: SFMsaa,
}
#[derive(Default, Clone, Serialize, Deserialize)]
pub enum SFMsaa {
    Off,
    Sample2,
    #[default]
    Sample4,
    Sample8,
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
#[derive(Serialize, Deserialize, Default, Clone)]
pub enum SFFxaa {
    Low,
    Medium,
    #[default]
    High,
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
#[derive(Serialize, Deserialize, Default, Clone)]
pub enum SFWindowMode {
    #[default]
    Windowed,
    BorderlessFullscreen,
    SizedFullscreen,
    Fullscreen,
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

impl Default for GraphicsSettings {
    fn default() -> Self {
        let default_res = WindowResolution::default();
        Self {
            resolution: (default_res.physical_width(), default_res.physical_height()),
            window_mode: SFWindowMode::default(),
            vsync: false,
            fxaa: Some(SFFxaa::default()),
            msaa: SFMsaa::Off,
        }
    }
}

pub(crate) fn setup_graphics_settings(
    mut settings: ResMut<GraphicsSettings>,
    mut res_events: EventWriter<SetResolution>,
    mut vsync_events: EventWriter<SetVsync>,
    mut w_mode_events: EventWriter<SetWindowMode>,
    mut fxaa_events: EventWriter<SetFxaa>,
    mut msaa_events: EventWriter<SetMsaa>,
) {
    let path = Path::new("data").join("settings").join("graphics.ron");

    let mut generate_new_config = !path.exists();

    if path.exists() {
        let settings_ron = fs::read_to_string(path.clone()).unwrap();
        match ron::from_str(&settings_ron) {
            Ok(s) => *settings = s,
            Err(_) => {
                generate_new_config = true;
            }
        }
    }

    if generate_new_config {
        let settings_ron = ron::ser::to_string_pretty(&*settings, PrettyConfig::default()).unwrap();
        fs::write(path, settings_ron).unwrap();
    }

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
}

pub struct SetResolution {
    pub resolution: (u32, u32),
}

pub(crate) fn set_resolution(
    mut events: EventReader<SetResolution>,
    mut primary_query: Query<&mut Window, With<PrimaryWindow>>,
    mut settings: ResMut<GraphicsSettings>,
) {
    for event in events.iter() {
        let mut primary = primary_query.get_single_mut().unwrap();
        primary
            .resolution
            .set(event.resolution.0 as f32, event.resolution.1 as f32);
        settings.resolution = (event.resolution.0, event.resolution.1);
    }
}
pub struct SetVsync {
    pub enabled: bool,
}

pub(crate) fn set_vsync(
    mut events: EventReader<SetVsync>,
    mut primary_query: Query<&mut Window, With<PrimaryWindow>>,
    mut settings: ResMut<GraphicsSettings>,
) {
    for event in events.iter() {
        let mut primary = primary_query.get_single_mut().unwrap();
        if event.enabled {
            primary.present_mode = PresentMode::AutoVsync;
        } else {
            primary.present_mode = PresentMode::AutoNoVsync;
        }
        settings.vsync = event.enabled;
    }
}

pub struct SetWindowMode {
    pub window_mode: SFWindowMode,
}
pub(crate) fn set_window_mode(
    mut events: EventReader<SetWindowMode>,
    mut primary_query: Query<&mut Window, With<PrimaryWindow>>,
    mut settings: ResMut<GraphicsSettings>,
) {
    for event in events.iter() {
        let mut primary = primary_query.get_single_mut().unwrap();

        primary.mode = event.window_mode.to_window_mode();

        settings.window_mode = event.window_mode.clone();
    }
}
pub struct SetFxaa {
    pub mode: Option<SFFxaa>,
}
pub(crate) fn set_fxaa(
    mut events: EventReader<SetFxaa>,
    mut settings: ResMut<GraphicsSettings>,
    mut query: Query<&mut Fxaa>,
) {
    for event in events.iter() {
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
pub struct SetMsaa {
    pub mode: SFMsaa,
}
pub(crate) fn set_msaa(
    mut events: EventReader<SetMsaa>,
    mut settings: ResMut<GraphicsSettings>,
    mut msaa: ResMut<Msaa>,
) {
    for event in events.iter() {
        settings.msaa = event.mode.clone();
        *msaa = settings.msaa.to_msaa();
    }
}
