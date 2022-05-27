use bevy_ecs::prelude::Component;

#[derive(Component, Default)]
pub struct ConstructionTool {
    pub construction_option: Option<String>,
}
