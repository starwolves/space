use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct RawEntity {
    pub entity_type: String,
    pub transform: String,
    pub data: String,
}
