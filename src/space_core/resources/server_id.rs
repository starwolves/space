use bevy::prelude::{Entity, FromWorld, World};

// Used for client, we can send this ID as an entityUpdate to the client which indicates it does not belong
// to a specific entity and it should be customly assigned to something such as UIs and other stuff which
// are not real server entities but just client GUI instances.
pub struct ServerId {
    pub id : Entity
}


impl FromWorld for ServerId {
    fn from_world(_world: &mut World) -> Self {
        ServerId {
           id : Entity::from_raw(0), 
        }
    }
}
