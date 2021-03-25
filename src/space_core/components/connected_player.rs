use bevy::prelude::Bundle;
#[derive(Bundle)]
pub struct ConnectedPlayer {
    pub handle : u32,
    pub authid : u16
}
