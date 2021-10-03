use std::collections::HashMap;

pub struct Examinable {
    pub assigned_texts : HashMap<u32, String>,
    pub name : String,
}

impl Default for Examinable {
    fn default() -> Self {
        Self {
            assigned_texts : HashMap::new(),
            name : "".to_string(),
        }
    }
}
