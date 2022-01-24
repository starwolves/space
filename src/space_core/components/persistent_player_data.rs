use bevy::prelude::Component;

#[derive(Clone, Component)]
pub struct PersistentPlayerData {
    pub user_name_is_set : bool,
    pub character_name : String,
    pub user_name : String,
}

impl Default for PersistentPlayerData {
    fn default() -> Self {
        Self {
            user_name_is_set : false,
            character_name: "".to_string(),
            user_name: "".to_string(),
        }
    }
}
