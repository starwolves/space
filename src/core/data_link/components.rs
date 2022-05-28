use bevy_ecs::prelude::Component;

#[derive(Component, Default)]
pub struct DataLink {
    pub links: Vec<DataLinkType>,
}

#[derive(PartialEq)]
pub enum DataLinkType {
    FullAtmospherics,
    RemoteLock,
    ShipEngineeringKnowledge,
}
