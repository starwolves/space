use bevy::prelude::Commands;
use bevy::prelude::EventReader;

use bevy::scene::SceneBundle;

use crate::grid::cell_id_to_world;
use crate::set_cell::SetCell;
use bevy::prelude::warn;
use bevy::prelude::Res;
use bevy::prelude::Transform;

use crate::grid::Gridmap;

pub(crate) fn set_cell_graphics(
    mut events: EventReader<SetCell>,
    gridmap_main: Res<Gridmap>,
    mut commands: Commands,
) {
    for set_cell in events.iter() {
        match gridmap_main
            .main_cell_properties
            .get(&set_cell.data.item_0.id)
        {
            Some(properties) => {
                commands.spawn(SceneBundle {
                    scene: properties.mesh_option.clone().unwrap(),
                    transform: Transform::from_translation(cell_id_to_world(set_cell.id)),
                    ..Default::default()
                });
            }
            None => {
                warn!("Couldnt find maincellproperties!");
            }
        }
    }
}
