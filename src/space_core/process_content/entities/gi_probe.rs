use serde::{Deserialize};

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
