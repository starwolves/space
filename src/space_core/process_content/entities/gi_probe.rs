use serde::{Deserialize};

use crate::space_core::components::gi_probe::GIProbe;

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct ExportData {
    bias : f32,
    compressed : bool,
    dynamic_range : u8,
    energy : f32,
    interior : bool,
    normal_bias : f32,
    propagation : f32,
    subdiv : u8
}

impl ExportData {
    pub fn to_component(self) -> GIProbe {
        GIProbe {
            bias : self.bias,
            compressed : self.compressed,
            dynamic_range : self.dynamic_range,
            energy : self.energy,
            interior : self.interior,
            normal_bias : self.normal_bias,
            propagation : self.propagation,
            subdiv : self.subdiv
        }
    }
}