use bevy::prelude::{Entity, Component};
#[derive(Component)]
pub struct CounterWindowSensor {

    pub parent : Entity

}

impl Default for CounterWindowSensor {
    fn default() -> Self {
        Self {
            parent : Entity::from_raw(0),
        }
    }
}
