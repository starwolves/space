use bevy::prelude::Color;
use serde::{Deserialize};
use crate::space_core::{
    functions::string_to_type_converters::{string_color_to_color},
    components::{
        omni_light::OmniLight
    }
};

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct ExportDataRaw {
    omni_attenuation : f32,
    omni_range : f32,
    omni_shadow_detail : u8,
    omni_shadow_mode : u8,
    bake_mode : u8,
    color : String,
    cull_mask : i64,
    light_energy : f32,
    light_indirect_energy : f32,
    negative : bool,
    light_specular : f32,
    shadow_bias : f32,
    shadow_color : String,
    shadow_contact : f32,
    shadow : bool,
    shadow_reverse_cull_face : bool
}

impl ExportData {

    pub fn new(raw : ExportDataRaw) -> ExportData {
        ExportData {
            omni_attenuation : raw.omni_attenuation,
            omni_range : raw.omni_range,
            omni_shadow_detail : raw.omni_shadow_detail,
            omni_shadow_mode : raw.omni_shadow_mode,
            bake_mode : raw.bake_mode,
            color : string_color_to_color(&raw.color),
            cull_mask : raw.cull_mask,
            light_energy : raw.light_energy,
            light_indirect_energy : raw.light_indirect_energy,
            negative : raw.negative,
            light_specular : raw.light_specular,
            shadow_bias : raw.shadow_bias,
            shadow_color : string_color_to_color(&raw.shadow_color),
            shadow_contact : raw.shadow_contact,
            shadow : raw.shadow,
            shadow_reverse_cull_face : raw.shadow_reverse_cull_face
        }
    }

    pub fn to_component(self) -> OmniLight {

        OmniLight{
            omni_attenuation : self.omni_attenuation,
            omni_range : self.omni_range,
            omni_shadow_detail : self.omni_shadow_detail,
            omni_shadow_mode : self.omni_shadow_mode,
            bake_mode : self.bake_mode,
            color : self.color,
            cull_mask : self.cull_mask,
            light_energy : self.light_energy,
            light_indirect_energy : self.light_indirect_energy,
            negative : self.negative,
            light_specular : self.light_specular,
            shadow_bias : self.shadow_bias,
            shadow_color : self.shadow_color,
            shadow_contact : self.shadow_contact,
            shadow : self.shadow,
            shadow_reverse_cull_face : self.shadow_reverse_cull_face
        }

    }

}

pub struct ExportData {
    omni_attenuation : f32,
    omni_range : f32,
    omni_shadow_detail : u8,
    omni_shadow_mode : u8,
    bake_mode : u8,
    color : Color,
    cull_mask : i64,
    light_energy : f32,
    light_indirect_energy : f32,
    negative : bool,
    light_specular : f32,
    shadow_bias : f32,
    shadow_color : Color,
    shadow_contact : f32,
    shadow : bool,
    shadow_reverse_cull_face : bool
}
