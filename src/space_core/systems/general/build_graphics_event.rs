use bevy::prelude::{EventReader, Query, info};

use crate::space_core::{components::{gi_probe::GIProbe, reflection_probe::ReflectionProbe}, events::general::build_graphics::BuildGraphics};


pub fn build_graphics_event(
    mut build_graphics_events: EventReader<BuildGraphics>,
    reflection_probe_query : Query<&ReflectionProbe>,
    gi_probe_query : Query<&GIProbe>
) {

    for build_graphics_event in build_graphics_events.iter() {

        let mut gi_probe_amount = 0;

        for gi_probe_component in gi_probe_query.iter() {

            gi_probe_amount+=1;

        }

        let mut reflection_probe_amount = 0;

        for reflection_probe_component in reflection_probe_query.iter() {

            reflection_probe_amount+=1;

        }

        info!("ReflectionProbes amount: {}", reflection_probe_amount);
        info!("GIProbes amount: {}", gi_probe_amount);

    }

}
