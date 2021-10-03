use std::collections::HashMap;

pub struct Examinable {
    pub examinable_text : String,
    pub assigned_texts : HashMap<u32, String>,
    pub name : String,
    pub custom_generator : bool,
}

impl Default for Examinable {
    fn default() -> Self {
        Self {
            examinable_text : "".to_string(),
            assigned_texts : HashMap::new(),
            name : "".to_string(),
            custom_generator : false,
        }
    }
}
