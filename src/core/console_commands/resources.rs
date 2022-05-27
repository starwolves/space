use crate::core::networking::resources::ConsoleCommandVariant;

#[derive(Default)]
pub struct AllConsoleCommands {
    pub list: Vec<(String, String, Vec<(String, ConsoleCommandVariant)>)>,
}
