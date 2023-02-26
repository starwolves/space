use bevy::prelude::{Entity, Resource};

#[derive(Default, Resource)]
pub struct TextInput {
    pub focused_input: Option<Entity>,
}
