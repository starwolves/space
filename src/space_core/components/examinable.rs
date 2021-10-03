use std::collections::BTreeMap;

pub struct Examinable {
    pub assigned_texts : BTreeMap<u32, String>,
    pub name : String,
}

impl Default for Examinable {
    fn default() -> Self {
        Self {
            assigned_texts : BTreeMap::new(),
            name : "".to_string(),
        }
    }
}
