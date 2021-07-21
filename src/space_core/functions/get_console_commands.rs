use crate::space_core::structs::network_messages::ConsoleCommandVariant;

pub fn get_console_commands() -> Vec<(String, String, Vec<(String, ConsoleCommandVariant)>)> {

    vec![
        (
            "rcon".to_string(),
            "For server administrators only. Obtainig rcon status allows for usage of rcon_* commands".to_string(),
            vec![
                (   
                    "password".to_string(),
                    ConsoleCommandVariant::String
                ),
            ]
        ),
    ]

}
