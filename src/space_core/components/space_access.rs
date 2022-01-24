use bevy::prelude::Component;

use super::pawn::SpaceAccessEnum;

#[derive(Component)]
pub struct SpaceAccess {
    pub access : Vec<SpaceAccessEnum>
}
