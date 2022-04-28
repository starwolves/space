use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct RawEntity {
    pub entity_type: String,
    pub transform: String,
    pub data: String,
}
