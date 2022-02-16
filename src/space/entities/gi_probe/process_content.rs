use serde::Deserialize;

use crate::space::core::entity::functions::string_to_type_converters::string_vec3_to_vec3;

use super::components::GIProbe;

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct ExportData {
    bias: f32,
    compressed: bool,
    dynamic_range: u8,
    energy: f32,
    interior: bool,
    normal_bias: f32,
    propagation: f32,
    subdiv: u8,
    extents: String,
}

impl ExportData {
    pub fn to_component(self) -> GIProbe {
        GIProbe {
            bias: self.bias,
            compressed: self.compressed,
            dynamic_range: self.dynamic_range,
            energy: self.energy,
            interior: self.interior,
            normal_bias: self.normal_bias,
            propagation: self.propagation,
            subdiv: self.subdiv,
            extents: string_vec3_to_vec3(&self.extents),
        }
    }
}
