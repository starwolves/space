use bevy_internal::{prelude::Component, math::Vec2};


#[derive(Component)]
pub struct Map {
    pub display_mode: Option<String>,
    pub available_display_modes: Vec<(String, String)>,
    pub view_range: usize,
    pub camera_position: Vec2,
    pub passed_mouse_cell: Option<(i16, i16)>,
}

impl Default for Map {
    fn default() -> Self {
        Self {
            display_mode: None,
            available_display_modes: vec![("Standard".to_string(), "standard".to_string())],
            view_range: 20,
            camera_position: Vec2::default(),
            passed_mouse_cell: None,
        }
    }
}
