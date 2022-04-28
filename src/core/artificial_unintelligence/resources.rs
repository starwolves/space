use bevy_math::Vec2;

use super::functions::pathing_et_steering::build_mapped_vectors;

// 4(maybe 3 *untested) is the absolute minimum if you want movement to be possible in every direction.
// The higher the value the more smooth the Ai's movement wil be,
// but the performance cost of the steering algorithim increases proportionally to the value.
pub const CONTEXT_MAP_RESOLUTION: usize = 8;

pub struct ContextMapVectors {
    pub context_map_vectors: [Vec2; CONTEXT_MAP_RESOLUTION],
}

impl Default for ContextMapVectors {
    fn default() -> Self {
        Self {
            context_map_vectors: build_mapped_vectors(),
        }
    }
}
