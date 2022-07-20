use bevy::prelude::{Res, ResMut};
use networking::messages::ExamineEntityMessages;
use shared::{
    data::Vec3Int,
    examinable::InputExamineEntity,
    gridmap::{ExamineMapMessage, GridmapExamineMessages},
    tab_actions::QueuedTabActions,
};

pub fn actions(
    queue: Res<QueuedTabActions>,

    mut event_examine_entity: ResMut<ExamineEntityMessages>,
    mut event_examine_map: ResMut<GridmapExamineMessages>,
) {
    for queued in queue.queue.iter() {
        if queued.tab_id == "actions::pawn/examine" && queued.handle_option.is_some() {
            match queued.target_entity_option {
                Some(entity_bits) => {
                    event_examine_entity.messages.push(InputExamineEntity {
                        handle: queued.handle_option.unwrap(),
                        examine_entity_bits: entity_bits,
                        entity: queued.player_entity,
                        ..Default::default()
                    });
                }
                None => match &queued.target_cell_option {
                    Some((gridmap_type, idx, idy, idz)) => {
                        event_examine_map.messages.push(ExamineMapMessage {
                            handle: queued.handle_option.unwrap(),
                            entity: queued.player_entity,
                            gridmap_type: gridmap_type.clone(),
                            gridmap_cell_id: Vec3Int {
                                x: *idx,
                                y: *idy,
                                z: *idz,
                            },
                            ..Default::default()
                        });
                    }
                    None => {}
                },
            }
        }
    }
}
