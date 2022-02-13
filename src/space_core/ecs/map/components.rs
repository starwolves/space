use bevy::{prelude::Component, math::Vec2};


#[derive(Component)]
pub struct Map {
    pub display_mode : Option<String>,
    pub available_display_modes : Vec<(String, String)>,
    pub view_range : f32,
    pub camera_position : Vec2,
}

impl Default for Map {
    fn default() -> Self {
        Self {
            display_mode : None,
            available_display_modes : vec![("Standard".to_string(),"standard".to_string())],
            view_range : 10.,
            camera_position: Vec2::default()
        }
    }
}
