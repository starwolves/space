use resources::is_server::is_server;
#[cfg(any(feature = "client", feature = "server"))]
pub fn register_basic_console_commands_for_type<T: EntityType + Clone + Default + 'static>(
    app: &mut App,
) {
    if is_server() {
        app.add_event::<RconSpawnEntity<T>>()
            .add_system(rcon_entity_console_commands::<T>.after(BuildingLabels::DefaultBuild));
    }
}
use bevy::prelude::App;
use entity::entity_types::EntityType;
use resources::labels::BuildingLabels;

use crate::commands::{
    inventory_item_console_commands, rcon_entity_console_commands, rcon_spawn_entity,
    rcon_spawn_held_entity, RconSpawnEntity, RconSpawnHeldEntity,
};
use bevy::prelude::IntoSystemDescriptor;

#[cfg(any(feature = "client", feature = "server"))]
pub fn register_basic_console_commands_for_inventory_item_type<
    T: EntityType + Clone + Default + 'static,
>(
    app: &mut App,
) {
    if is_server() {
        app.add_event::<RconSpawnEntity<T>>()
            .add_system(rcon_entity_console_commands::<T>.after(BuildingLabels::DefaultBuild))
            .add_system(rcon_spawn_entity::<T>)
            .add_system(rcon_spawn_held_entity::<T>)
            .add_system(
                inventory_item_console_commands::<T>
                    .before(BuildingLabels::TriggerBuild)
                    .label(BuildingLabels::NormalBuild),
            )
            .add_event::<RconSpawnHeldEntity<T>>();
    }
}
