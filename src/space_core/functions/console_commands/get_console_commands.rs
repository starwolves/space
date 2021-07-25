use crate::space_core::resources::network_messages::ConsoleCommandVariant;


pub fn get_console_commands() -> Vec<(String, String, Vec<(String, ConsoleCommandVariant)>)> {

    vec![
        (
            "rcon".to_string(),
            "For server administrators only. Obtaining rcon status allows for usage of rcon_* commands".to_string(),
            vec![
                (   
                    "password".to_string(),
                    ConsoleCommandVariant::String
                ),
            ]
        ),
        (
            "rcon_status".to_string(),
            "For server administrators only. Check if the server has granted you the RCON status.".to_string(),
            vec![]
        ),
        (
            "rcon_spawn_entity".to_string(),
            "For server administrators only. Spawn in entities in your proximity.".to_string(),
            vec![
                (
                    "entity_name".to_string(),
                    ConsoleCommandVariant::String
                ),
                (
                    "amount".to_string(),
                    ConsoleCommandVariant::Int
                ),
            ]
        )
    ]

}
