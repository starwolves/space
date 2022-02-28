use bevy_internal::prelude::Component;


#[derive(Component)]
pub struct ConstructionTool {
    pub construction_option: Option<String>,
}

impl Default for ConstructionTool {
    fn default() -> Self {
        Self {
            construction_option: None,
        }
    }
}
