use std::collections::BTreeMap;

pub struct Examinable {
    pub assigned_texts : BTreeMap<u32, String>,
    pub a_name : String,
    pub name : String,
}

impl Default for Examinable {
    fn default() -> Self {
        Self {
            assigned_texts : BTreeMap::new(),
            a_name : "".to_string(),
            name : "".to_string(),
        }
    }
}
