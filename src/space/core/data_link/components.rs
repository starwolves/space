use bevy_ecs::prelude::Component;

#[derive(Component)]
pub struct DataLink {
    pub links: Vec<DataLinkType>,
}

impl Default for DataLink {
    fn default() -> Self {
        Self { links: vec![] }
    }
}

#[derive(PartialEq)]
pub enum DataLinkType {
    FullAtmospherics,
    RemoteLock,
    ShipEngineeringKnowledge,
}
