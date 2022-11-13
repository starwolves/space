use bevy::prelude::Component;

/// The component for entities with data links.
#[derive(Component, Default)]
#[cfg(feature = "server")]
pub struct DataLink {
    pub links: Vec<DataLinkType>,
}

#[derive(PartialEq)]
#[cfg(feature = "server")]
pub enum DataLinkType {
    FullAtmospherics,
    RemoteLock,
    ShipEngineeringKnowledge,
}
