use bevy::prelude::Entity;

pub struct Sensable{
    pub is_light : bool,
    pub is_audible : bool,
    pub sensed_by : Vec<Entity>,
    pub sensed_by_cached : Vec<Entity>
}
