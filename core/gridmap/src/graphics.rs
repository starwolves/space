use actions::core::TargetCell;
use bevy::prelude::Commands;
use bevy::prelude::EventReader;

use bevy::scene::SceneBundle;

use bevy::prelude::warn;
use bevy::prelude::Res;

use crate::grid::AddTile;
use crate::grid::Gridmap;

pub(crate) fn set_cell_graphics(
    mut events: EventReader<AddTile>,
    gridmap_main: Res<Gridmap>,
    mut commands: Commands,
) {
    for set_cell in events.iter() {
        match gridmap_main.main_cell_properties.get(&set_cell.tile_type) {
            Some(properties) => {
                let transform = gridmap_main.get_cell_transform(
                    TargetCell {
                        id: set_cell.id,
                        face: set_cell.face.clone(),
                    },
                    set_cell.orientation,
                );

                commands.entity(set_cell.entity).insert(SceneBundle {
                    scene: properties.mesh_option.clone().unwrap(),
                    transform,
                    ..Default::default()
                });
            }
            None => {
                warn!("Couldnt find maincellproperties!");
            }
        }
    }
}
