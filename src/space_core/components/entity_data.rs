pub struct EntityData {
    pub entity_class : String,
    pub entity_type : String,
    pub entity_group : EntityGroup,
}


#[derive(Copy, Clone)]
pub enum EntityGroup {
    None,
    AirLock,
    CounterWindowSensor,
    Pawn
}

impl Default for EntityData {
    fn default() -> Self {
        Self {
            entity_class : "".to_string(),
            entity_type : "".to_string(),
            entity_group: EntityGroup::None,
        }
    }
}
