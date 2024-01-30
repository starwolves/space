use console_commands::commands::ConsoleCommandsSet;
use resources::{
    modes::is_server_mode,
    ordering::{BuildingSet, Update},
    plugin::SpawnItemSet,
};
pub fn register_basic_console_commands_for_type<T: EntityType + Clone + Default + 'static>(
    app: &mut App,
) {
    if is_server_mode(app) {
        app.add_event::<RconSpawnEntity<T>>()
            .add_systems(Update, rcon_entity_console_commands::<T>);
    }
}
use bevy::prelude::{App, IntoSystemConfigs};
use entity::entity_types::EntityType;

use crate::commands::{
    rcon_entity_console_commands, rcon_spawn_entity, RconSpawnEntity, RconSpawnHeldEntity,
};

pub fn register_basic_console_commands_for_inventory_item_type<
    T: EntityType + Clone + Default + 'static,
>(
    app: &mut App,
) {
    if is_server_mode(app) {
        app.add_event::<RconSpawnEntity<T>>()
            .add_systems(
                Update,
                (
                    rcon_entity_console_commands::<T>.after(ConsoleCommandsSet::Input),
                    rcon_spawn_entity::<T>
                        .before(SpawnItemSet::SpawnHeldItem)
                        .in_set(BuildingSet::TriggerBuild),
                ),
            )
            .add_event::<RconSpawnHeldEntity<T>>();
    }
}
