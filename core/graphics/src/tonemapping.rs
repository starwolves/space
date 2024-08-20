use std::collections::HashMap;

use bevy::{
    core_pipeline::tonemapping::Tonemapping,
    prelude::{ButtonInput, KeyCode, Query, Res, Resource},
    render::view::{ColorGrading, ColorGradingGlobal},
};

#[derive(Resource)]
pub struct PerMethodSettings {
    pub settings: HashMap<Tonemapping, ColorGrading>,
}

impl PerMethodSettings {
    fn basic_scene_recommendation(method: Tonemapping) -> ColorGrading {
        let mut r = ColorGrading::default();
        match method {
            Tonemapping::Reinhard | Tonemapping::ReinhardLuminance => {
                r.global = ColorGradingGlobal {
                    exposure: 0.5,
                    ..Default::default()
                };

                r
            }
            Tonemapping::AcesFitted => {
                r.global = ColorGradingGlobal {
                    exposure: 0.35,
                    ..Default::default()
                };

                r
            }
            Tonemapping::AgX => {
                for section in r.all_sections_mut() {
                    section.gamma = 1.0;
                    // `pre_saturation` has been renamed to `saturation`.
                    section.saturation = 1.1;
                }
                r.global = ColorGradingGlobal {
                    exposure: 0.5,
                    ..Default::default()
                };

                r
            }
            _ => ColorGrading::default(),
        }
    }
}

impl Default for PerMethodSettings {
    fn default() -> Self {
        let mut settings = HashMap::new();

        for method in [
            Tonemapping::None,
            Tonemapping::Reinhard,
            Tonemapping::ReinhardLuminance,
            Tonemapping::AcesFitted,
            Tonemapping::AgX,
            Tonemapping::SomewhatBoringDisplayTransform,
            Tonemapping::TonyMcMapface,
            Tonemapping::BlenderFilmic,
        ] {
            settings.insert(
                method,
                PerMethodSettings::basic_scene_recommendation(method),
            );
        }

        Self { settings }
    }
}

pub fn toggle_tonemapping_method(
    keys: Res<ButtonInput<KeyCode>>,
    mut tonemapping: Query<&mut Tonemapping>,
    mut color_grading: Query<&mut ColorGrading>,
    per_method_settings: Res<PerMethodSettings>,
) {
    for mut method in tonemapping.iter_mut() {
        for mut color_grading in color_grading.iter_mut() {
            if keys.just_pressed(KeyCode::Digit1) {
                *method = Tonemapping::None;
            } else if keys.just_pressed(KeyCode::Digit2) {
                *method = Tonemapping::Reinhard;
            } else if keys.just_pressed(KeyCode::Digit3) {
                *method = Tonemapping::ReinhardLuminance;
            } else if keys.just_pressed(KeyCode::Digit4) {
                *method = Tonemapping::AcesFitted;
            } else if keys.just_pressed(KeyCode::Digit5) {
                *method = Tonemapping::AgX;
            } else if keys.just_pressed(KeyCode::Digit6) {
                *method = Tonemapping::SomewhatBoringDisplayTransform;
            } else if keys.just_pressed(KeyCode::Digit7) {
                *method = Tonemapping::TonyMcMapface;
            } else if keys.just_pressed(KeyCode::Digit8) {
                *method = Tonemapping::BlenderFilmic;
            }

            *color_grading = per_method_settings
                .settings
                .get::<Tonemapping>(&method)
                .unwrap()
                .clone();
        }
    }
}
