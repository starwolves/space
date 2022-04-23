use bevy_app::EventWriter;
use bevy_ecs::system::Res;

use crate::space::core::tab_actions::resources::QueuedTabActions;

use super::events::{InputConstruct, InputConstructionOptions, InputDeconstruct};
pub fn construction_tool_actions(
    queue: Res<QueuedTabActions>,
    mut event_construct: EventWriter<InputConstruct>,
    mut event_construction_options: EventWriter<InputConstructionOptions>,
    mut event_deconstruct: EventWriter<InputDeconstruct>,
) {
    for queued in queue.queue.iter() {
        if queued.tab_id == "construct" {
            if queued.target_cell_option.is_some() {
                event_construct.send(InputConstruct {
                    handle: queued.handle,
                    target_cell: queued.target_cell_option.as_ref().unwrap().clone(),
                    belonging_entity: queued.belonging_entity_option.unwrap(),
                });
            }
        } else if queued.tab_id == "constructionoptions" {
            event_construction_options.send(InputConstructionOptions {
                handle: queued.handle,
                belonging_entity: queued.belonging_entity_option.unwrap(),
            });
        } else if queued.tab_id == "deconstruct" {
            if queued.target_entity_option.is_some() || queued.target_cell_option.is_some() {
                event_deconstruct.send(InputDeconstruct {
                    handle: queued.handle,
                    target_cell_option: queued.target_cell_option.clone(),
                    target_entity_option: queued.target_entity_option,
                    belonging_entity: queued.belonging_entity_option.unwrap(),
                });
            }
        }
    }
}
