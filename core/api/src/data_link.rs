use bevy::prelude::Component;

/// The component for entities with data links.
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
