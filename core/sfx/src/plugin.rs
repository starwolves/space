use std::time::Duration;

use bevy::{
    prelude::{App, FixedUpdate, IntoSystemConfigs, Plugin, PostUpdate, Update},
    time::common_conditions::on_fixed_timer,
};
use entity::{entity_data::InterpolationSet, entity_types::register_entity_type};
use networking::messaging::{register_reliable_message, MessageSender};
use resources::{is_server::is_server, labels::PostUpdateLabels};

use crate::{
    builder::{AmbienceSfxEntityType, RepeatingSfxEntityType, SfxEntityType},
    entity_update::SfxAutoDestroyTimers,
    net::SfxServerMessage,
    timers::free_sfx,
};

use super::{
    entity_update::{repeating_sfx_update, sfx_update},
    timers::tick_timers_slowed,
};

pub struct SfxPlugin;

impl Plugin for SfxPlugin {
    fn build(&self, app: &mut App) {
        if is_server() {
            app.add_systems(
                FixedUpdate,
                tick_timers_slowed
                    .in_set(InterpolationSet::Main)
                    .run_if(on_fixed_timer(Duration::from_secs_f32(1. / 2.))),
            )
            .add_systems(
                PostUpdate,
                (
                    sfx_update.in_set(PostUpdateLabels::EntityUpdate),
                    repeating_sfx_update.in_set(PostUpdateLabels::EntityUpdate),
                ),
            )
            .add_systems(Update, free_sfx)
            .init_resource::<SfxAutoDestroyTimers>();
        }
        register_reliable_message::<SfxServerMessage>(app, MessageSender::Server);
        register_entity_type::<AmbienceSfxEntityType>(app);
        register_entity_type::<RepeatingSfxEntityType>(app);
        register_entity_type::<SfxEntityType>(app);
    }
}
