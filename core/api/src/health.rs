/// Health data for structures like gridmap cells.
#[derive(Clone, Default)]
pub struct StructureHealth {
    pub brute: f32,
    pub burn: f32,
    pub toxin: f32,
}

/// The health data for entities.
#[derive(Default, Clone)]
pub struct EntityContainer {
    pub brute: f32,
    pub burn: f32,
    pub toxin: f32,
}
