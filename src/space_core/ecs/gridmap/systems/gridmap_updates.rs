
use bevy::prelude::{ResMut, Query, Entity, EventWriter};

use crate::space_core::{ecs::{pawn::components::{ConnectedPlayer, Senser}, gridmap::{events::NetGridmapUpdates, resources::{GridmapMain, GridmapDetails1, to_doryen_coordinates}}, networking::resources::{ReliableServerMessage, GridMapType}}};

pub fn gridmap_updates(

    mut gridmap_main : ResMut<GridmapMain>,
    mut gridmap_details1 : ResMut<GridmapDetails1>,
    sensers : Query<(Entity,&Senser, &ConnectedPlayer)>,
    mut net_gridmap_updates : EventWriter<NetGridmapUpdates>,

) {

    for (cell_id, cell_update) in gridmap_main.updates.iter_mut() {

        let cell_coords = to_doryen_coordinates(cell_id.x, cell_id.z);

        for (senser_entity, senser_component, connected_player_component) in sensers.iter() {

            if connected_player_component.connected && !cell_update.entities_received.contains(&senser_entity) && senser_component.fov.is_in_fov(cell_coords.0, cell_coords.1) {
                
                cell_update.entities_received.push(senser_entity);
                if cell_update.cell_data.item != -1 {
                    net_gridmap_updates.send(NetGridmapUpdates {
                        handle: connected_player_component.handle,
                        message: ReliableServerMessage::AddCell(cell_id.x, cell_id.y, cell_id.z, cell_update.cell_data.item, cell_update.cell_data.orientation, GridMapType::Main),
                    });
                } else {
                    net_gridmap_updates.send(NetGridmapUpdates {
                        handle: connected_player_component.handle,
                        message: ReliableServerMessage::RemoveCell(cell_id.x, cell_id.y, cell_id.z, GridMapType::Main),
                    });
                }

            }

        }

    }

    for (cell_id, cell_update) in gridmap_details1.updates.iter_mut() {

        let cell_coords = to_doryen_coordinates(cell_id.x, cell_id.z);

        for (senser_entity, senser_component, connected_player_component) in sensers.iter() {

            if connected_player_component.connected && !cell_update.entities_received.contains(&senser_entity) && senser_component.fov.is_in_fov(cell_coords.0, cell_coords.1) {
                
                cell_update.entities_received.push(senser_entity);
                if cell_update.cell_data.item != -1 {
                    net_gridmap_updates.send(NetGridmapUpdates {
                        handle: connected_player_component.handle,
                        message: ReliableServerMessage::AddCell(cell_id.x, cell_id.y, cell_id.z, cell_update.cell_data.item, cell_update.cell_data.orientation, GridMapType::Details1),
                    });
                } else {
                    net_gridmap_updates.send(NetGridmapUpdates {
                        handle: connected_player_component.handle,
                        message: ReliableServerMessage::RemoveCell(cell_id.x, cell_id.y, cell_id.z, GridMapType::Details1),
                    });
                }

            }

        }

    }

}
