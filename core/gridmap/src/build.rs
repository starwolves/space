use bevy::{
    hierarchy::BuildChildren,
    prelude::{warn, Commands, Entity, GlobalTransform, Transform},
};
use bevy_rapier3d::prelude::{CollisionGroups, Friction, Group, RigidBody};
use math::grid::{cell_id_to_world, Vec3Int};

use crate::grid::Gridmap;
use crate::grid::{Cell, Orientation};
use physics::physics::{get_bit_masks, ColliderGroup};

/// Spawn a main gridmap cell as a function.
pub fn spawn_main_cell(
    commands: &mut Commands,
    cell_id: Vec3Int,
    cell_item_id: u16,
    _cell_rotation: &Option<Orientation>,
    gridmap_data: &Gridmap,
) -> Entity {
    let cell_properties;
    match gridmap_data.main_cell_properties.get(&cell_item_id) {
        Some(x) => {
            cell_properties = x;
        }
        None => {
            warn!("Unknown cellid {}. Initialization of gridmap cell in startup gridmap systems missing.", cell_item_id);
            return Entity::from_bits(0);
        }
    }

    let mut world_position = Transform::from_translation(cell_id_to_world(cell_id));
    world_position.translation += cell_properties.collider_position.translation;
    world_position.rotation *= cell_properties.collider_position.rotation;

    let mut entity_builder = commands.spawn(());
    entity_builder
        .insert(RigidBody::Fixed)
        .insert(GlobalTransform::default())
        .insert(world_position)
        .insert(Cell { id: cell_id });

    let entity_id = entity_builder.id();

    let masks = get_bit_masks(ColliderGroup::Standard);

    let mut friction_component = Friction::coefficient(cell_properties.friction);
    friction_component.combine_rule = cell_properties.combine_rule;

    entity_builder.with_children(|children| {
        children
            .spawn(())
            .insert(cell_properties.collider.clone())
            .insert(Transform::IDENTITY)
            .insert(friction_component)
            .insert(CollisionGroups::new(
                Group::from_bits(masks.0).unwrap(),
                Group::from_bits(masks.1).unwrap(),
            ));
    });

    entity_id
}
