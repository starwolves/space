use bevy::prelude::Entity;

pub struct CounterWindowSensor {

    pub parent : Entity

}

impl Default for CounterWindowSensor {
    fn default() -> Self {
        Self {
            parent : Entity::new(0),
        }
    }
}
