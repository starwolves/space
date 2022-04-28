use bevy_math::Vec3;
use serde::Deserialize;

use crate::core::entity::functions::string_to_type_converters::{
    string_color_to_color, string_vec3_to_vec3,
};

use super::components::ReflectionProbe;

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct ExportDataRaw {
    projection_enabled: bool,
    cull_mask: i64,
    shadows_enabled: bool,
    extents: String,
    intensity: f32,
    interior_ambient: String,
    interior_ambient_probe_contribution: f32,
    interior_ambient_energy: f32,
    set_as_interior: bool,
    max_distance: f32,
    origin_offset: String,
    update_mode: u8,
}

pub struct ExportData {
    projection_enabled: bool,
    cull_mask: i64,
    shadows_enabled: bool,
    extents: Vec3,
    intensity: f32,
    interior_ambient: (f32, f32, f32, f32),
    interior_ambient_probe_contribution: f32,
    interior_ambient_energy: f32,
    set_as_interior: bool,
    max_distance: f32,
    origin_offset: Vec3,
    update_mode: u8,
}

impl ExportData {
    pub fn new(raw: ExportDataRaw) -> ExportData {
        ExportData {
            projection_enabled: raw.projection_enabled,
            cull_mask: raw.cull_mask,
            shadows_enabled: raw.shadows_enabled,
            extents: string_vec3_to_vec3(&raw.extents),
            intensity: raw.intensity,
            interior_ambient: string_color_to_color(&raw.interior_ambient),
            interior_ambient_probe_contribution: raw.interior_ambient_probe_contribution,
            interior_ambient_energy: raw.interior_ambient_energy,
            set_as_interior: raw.set_as_interior,
            max_distance: raw.max_distance,
            origin_offset: string_vec3_to_vec3(&raw.origin_offset),
            update_mode: raw.update_mode,
        }
    }

    pub fn to_component(self) -> ReflectionProbe {
        ReflectionProbe {
            projection_enabled: self.projection_enabled,
            cull_mask: self.cull_mask,
            shadows_enabled: self.shadows_enabled,
            extents: self.extents,
            intensity: self.intensity,
            interior_ambient: self.interior_ambient,
            interior_ambient_probe_contribution: self.interior_ambient_probe_contribution,
            interior_ambient_energy: self.interior_ambient_energy,
            set_as_interior: self.set_as_interior,
            max_distance: self.max_distance,
            origin_offset: self.origin_offset,
            update_mode: self.update_mode,
        }
    }
}
