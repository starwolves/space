use bevy::prelude::Entity;

pub struct Visible{
    pub is_light : bool,
    pub sensed_by : Vec<Entity>,
    pub sensed_by_cached : Vec<Entity>
}
