pub struct Examinable {
    pub examinable_text : String,
    pub name : String,
}

impl Default for Examinable {
    fn default() -> Self {
        Self {
            examinable_text : "".to_string(),
            name : "".to_string(),
        }
    }
}
