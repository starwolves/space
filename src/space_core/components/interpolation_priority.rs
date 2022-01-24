use bevy::prelude::Component;

#[derive(Component)]
pub struct InterpolationPriority {
    pub priority : InterpolationPriorityStatus,
}

impl Default for InterpolationPriority {
    fn default() -> Self {
        Self {
            priority : InterpolationPriorityStatus::Low,
        }
    }
}


#[allow(dead_code)]
pub enum InterpolationPriorityStatus {
    High,
    Medium,
    Low
}
