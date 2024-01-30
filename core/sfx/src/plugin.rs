use std::time::Duration;

use bevy::{
    prelude::{App, IntoSystemConfigs, Plugin},
    time::common_conditions::on_timer,
};
use entity::{entity_data::InterpolationSet, entity_types::register_entity_type};
use networking::messaging::{register_reliable_message, MessageSender};
use resources::{modes::is_server_mode, ordering::Update};

use crate::{
    builder::{AmbienceSfxEntityType, RepeatingSfxEntityType, SfxEntityType},
    net::SfxServerMessage,
};

use super::timers::tick_timers_slowed;

pub struct SfxPlugin;

impl Plugin for SfxPlugin {
    fn build(&self, app: &mut App) {
        if is_server_mode(app) {
            app.add_systems(
                Update,
                tick_timers_slowed
                    .in_set(InterpolationSet::Main)
                    .run_if(on_timer(Duration::from_secs_f32(1. / 2.))),
            );
        }
        register_reliable_message::<SfxServerMessage>(app, MessageSender::Server, true);
        register_entity_type::<AmbienceSfxEntityType>(app);
        register_entity_type::<RepeatingSfxEntityType>(app);
        register_entity_type::<SfxEntityType>(app);
    }
}
