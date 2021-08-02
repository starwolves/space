pub struct Examinable {
    pub description : String,
    pub name : String,
}

impl Default for Examinable {
    fn default() -> Self {
        Self {
            description : "".to_string(),
            name : "".to_string(),
        }
    }
}
