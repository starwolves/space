pub struct EntityData {
    pub entity_class : String,
    pub entity_type : String,
    pub entity_group : EntityGroup
}


#[derive(Copy, Clone)]
pub enum EntityGroup {
    None,
    AirLock,
    CounterWindowSensor,
    Pawn
}
