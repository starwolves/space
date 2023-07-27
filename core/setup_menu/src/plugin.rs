use bevy::prelude::{App, FixedUpdate, IntoSystemConfigs, Plugin};
use controller::networking::InputUIInput;
use networking::{
    client::is_client_connected,
    messaging::{register_reliable_message, MessageSender},
};
use player::{boarding::done_boarding, connections::process_response, plugin::ConfigurationLabel};
use resources::{
    is_server::is_server,
    sets::{BuildingSet, MainSet},
};

use crate::{
    core::{
        client_setup_ui, configure, initialize_setupui, new_clients_enable_setupui,
        receive_input_character_name, setupui_loaded, ui_input_boarding, SetupUiState,
        SetupUiUserDataSets,
    },
    net::{SetupUiClientMessage, SetupUiServerMessage},
};
pub struct SetupMenuPlugin;

impl Plugin for SetupMenuPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.add_systems(
                FixedUpdate,
                (
                    ui_input_boarding.before(done_boarding),
                    initialize_setupui.in_set(BuildingSet::TriggerBuild),
                    configure
                        .in_set(ConfigurationLabel::Main)
                        .after(ConfigurationLabel::SpawnEntity)
                        .after(process_response),
                    new_clients_enable_setupui,
                    setupui_loaded,
                    receive_input_character_name,
                )
                    .in_set(MainSet::Update),
            )
            .add_event::<InputUIInput>()
            .init_resource::<SetupUiState>()
            .init_resource::<SetupUiUserDataSets>();
        } else {
            app.add_systems(
                FixedUpdate,
                client_setup_ui
                    .run_if(is_client_connected)
                    .in_set(MainSet::Update),
            );
        }

        register_reliable_message::<SetupUiServerMessage>(app, MessageSender::Server);
        register_reliable_message::<SetupUiClientMessage>(app, MessageSender::Client);
    }
}
