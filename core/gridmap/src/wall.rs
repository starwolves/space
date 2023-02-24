use bevy::prelude::{Commands, EventReader, EventWriter, ResMut};

use crate::grid::{AddGroup, AddTile, Gridmap};

pub(crate) fn add_wall_group(
    mut events: EventReader<AddGroup>,
    mut gridmap_main: ResMut<Gridmap>,
    mut set_tile: EventWriter<AddTile>,
    mut commands: Commands,
) {
    for add_group_event in events.iter() {
        let wall_id = *gridmap_main.main_name_id_map.get("generic_wall_1").unwrap();
        let group_instance_id = gridmap_main.group_instance_incremental;
        gridmap_main.group_instance_incremental += 1;
        set_tile.send(AddTile {
            id: add_group_event.id,
            tile_type: wall_id,
            orientation: add_group_event.orientation.clone(),
            face: add_group_event.face.clone(),
            group_instance_id_option: Some(group_instance_id),
            entity: commands.spawn(()).id(),
            default_map_spawn: add_group_event.default_map_spawn,
        });
        let mut high_id = add_group_event.id.clone();
        high_id.y += 1;
        set_tile.send(AddTile {
            id: high_id,
            tile_type: wall_id,
            orientation: add_group_event.orientation.clone(),
            face: add_group_event.face.clone(),
            group_instance_id_option: Some(group_instance_id),
            entity: commands.spawn(()).id(),
            default_map_spawn: add_group_event.default_map_spawn,
        });
    }
}
