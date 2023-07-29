use bevy::prelude::{Entity, Resource};

#[derive(Default, Resource, Debug)]
pub struct TextInput {
    pub focused_input: Option<Entity>,
    pub old_focus: Option<Entity>,
}

/// Resource containing the main menu state.
#[derive(Default, Resource)]

pub struct MainMenuState {
    pub enabled: bool,
    pub root: Option<Entity>,
    pub camera: Option<Entity>,
}
