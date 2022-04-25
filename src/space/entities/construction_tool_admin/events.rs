use bevy_app::EventWriter;
use bevy_ecs::{entity::Entity, system::Res};

use crate::space::core::{
    networking::resources::{GridMapType, ReliableServerMessage},
    tab_actions::resources::QueuedTabActions,
};

pub struct InputConstruct {
    pub handle_option: Option<u64>,
    pub target_cell: (GridMapType, i16, i16, i16),
    pub belonging_entity: u64,
}

pub struct InputConstructionOptionsSelection {
    pub handle_option: Option<u64>,
    pub menu_selection: String,
    // Entity has been validated.
    pub entity: Entity,
}

pub struct InputConstructionOptions {
    pub handle_option: Option<u64>,
    pub belonging_entity: u64,
}

pub struct InputDeconstruct {
    pub handle_option: Option<u64>,
    pub target_cell_option: Option<(GridMapType, i16, i16, i16)>,
    pub target_entity_option: Option<u64>,
    pub belonging_entity: u64,
}

pub struct NetConstructionTool {
    pub handle: u64,
    pub message: ReliableServerMessage,
}

pub fn construction_tool_actions(
    queue: Res<QueuedTabActions>,
    mut event_construct: EventWriter<InputConstruct>,
    mut event_construction_options: EventWriter<InputConstructionOptions>,
    mut event_deconstruct: EventWriter<InputDeconstruct>,
) {
    for queued in queue.queue.iter() {
        if queued.tab_id == "action::construction_tool_admin/construct" {
            if queued.target_cell_option.is_some() {
                event_construct.send(InputConstruct {
                    handle_option: queued.handle_option,
                    target_cell: queued.target_cell_option.as_ref().unwrap().clone(),
                    belonging_entity: queued.belonging_entity_option.unwrap(),
                });
            }
        } else if queued.tab_id == "action::construction_tool_admin/constructionoptions" {
            event_construction_options.send(InputConstructionOptions {
                handle_option: queued.handle_option,
                belonging_entity: queued.belonging_entity_option.unwrap(),
            });
        } else if queued.tab_id == "action::construction_tool_admin/deconstruct" {
            if queued.target_entity_option.is_some() || queued.target_cell_option.is_some() {
                event_deconstruct.send(InputDeconstruct {
                    handle_option: queued.handle_option,
                    target_cell_option: queued.target_cell_option.clone(),
                    target_entity_option: queued.target_entity_option,
                    belonging_entity: queued.belonging_entity_option.unwrap(),
                });
            }
        }
    }
}
