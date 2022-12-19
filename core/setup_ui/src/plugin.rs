use bevy::prelude::{App, IntoSystemDescriptor, Plugin};
use controller::networking::InputUIInput;
use iyes_loopless::prelude::IntoConditionalSystem;
use networking::{
    client::is_client_connected,
    messaging::{init_reliable_message, MessageSender},
};
use player::plugin::ConfigurationLabel;
use resources::{is_server::is_server, labels::SummoningLabels};

use crate::core::{
    client_setup_ui, configure, initialize_setupui, new_clients_enable_setupui,
    receive_input_character_name, setupui_loaded, ui_input_boarding, SetupUiClientMessage,
    SetupUiServerMessage, SetupUiState, SetupUiUserDataSets,
};
pub struct SetupUiPlugin;

impl Plugin for SetupUiPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.add_system(ui_input_boarding)
                .add_system(initialize_setupui.label(SummoningLabels::TriggerSummon))
                .add_event::<InputUIInput>()
                .add_system(
                    configure
                        .label(ConfigurationLabel::Main)
                        .after(ConfigurationLabel::SpawnEntity),
                )
                .add_system(new_clients_enable_setupui)
                .init_resource::<SetupUiState>()
                .add_system(setupui_loaded)
                .add_system(receive_input_character_name)
                .init_resource::<SetupUiUserDataSets>();
        } else {
            app.add_system(client_setup_ui.run_if(is_client_connected));
        }

        init_reliable_message::<SetupUiServerMessage>(app, MessageSender::Server);
        init_reliable_message::<SetupUiClientMessage>(app, MessageSender::Client);
    }
}
