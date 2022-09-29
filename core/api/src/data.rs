use std::collections::HashMap;

use bevy::{
    math::Vec3,
    prelude::{Component, Entity, SystemLabel},
};
use serde::{Deserialize, Serialize};

/// A resource that links entities to their appropiate connection handles for connected players.
#[derive(Default)]
pub struct HandleToEntity {
    pub map: HashMap<u64, Entity>,
    pub inv_map: HashMap<Entity, u64>,
}

/// The component for an entity controlled by a connected player.
#[derive(Component, Clone)]
pub struct ConnectedPlayer {
    pub handle: u64,
    pub authid: u16,
    pub rcon: bool,
    pub connected: bool,
}
impl Default for ConnectedPlayer {
    fn default() -> Self {
        Self {
            handle: 0,
            authid: 0,
            rcon: false,
            connected: true,
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum StartupLabels {
    ConsoleCommands,
    MiscResources,
    InitDefaultGridmapData,
    BuildGridmap,
    InitAtmospherics,
    ListenConnections,
    InitEntities,
    ServerIsLive,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum MapLabels {
    ChangeMode,
    MapInput,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum ActionsLabels {
    Clear,
    Init,
    Build,
    Approve,
    Action,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum PreUpdateLabels {
    NetEvents,
    ProcessInput,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum CombatLabels {
    RegisterAttacks,
    CacheAttack,
    WeaponHandler,
    Query,
    StartApplyDamage,
    FinalizeApplyDamage,
    DamageResults,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum UpdateLabels {
    ProcessMovementInput,
    DropCurrentItem,
    StandardCharacters,
    TextTreeInputSelection,
    DeconstructCell,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum PostUpdateLabels {
    EntityUpdate,
    SendEntityUpdates,
    VisibleChecker,
    Net,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum SummoningLabels {
    TriggerSummon,
    DefaultSummon,
    NormalSummon,
}

/// Resource containing the tickrate of the server loop.
pub struct TickRate {
    pub physics_rate: u8,
    pub bevy_rate: u8,
}

impl Default for TickRate {
    fn default() -> Self {
        TickRate {
            physics_rate: 24,
            bevy_rate: 64,
        }
    }
}

/// Resource used for client, we can send this ID as an entityUpdate to the client which indicates it does not belong
/// to a specific entity and it should be customly assigned to something such as UIs and other stuff which
/// are not real server entities but just client GUI instances.
pub struct ServerId {
    pub id: Entity,
}

impl Default for ServerId {
    fn default() -> Self {
        ServerId {
            id: Entity::from_raw(0),
        }
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug, Default)]
pub struct Vec2Int {
    pub x: i16,
    pub y: i16,
}

#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug, Serialize, Deserialize, Default)]
pub struct Vec3Int {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

#[derive(Clone)]
pub struct ShowcaseData {
    pub handle: u64,
}

/// Component for entities with zero gravity.
#[derive(Component)]
pub struct ZeroGravity;

/// Component for entities in the showcase.
#[derive(Component)]
pub struct Showcase {
    pub handle: u64,
}

pub const PISTOL_L1_ENTITY_NAME: &str = "pistolL1";
pub const JUMPSUIT_SECURITY_ENTITY_NAME: &str = "jumpsuitSecurity";

pub const HUMAN_DUMMY_ENTITY_NAME: &str = "humanDummy";
pub const HUMAN_MALE_ENTITY_NAME: &str = "humanMale";

/// Component holding Godot GIProbe properties.
#[derive(Component, Clone)]
pub struct GIProbe {
    pub bias: f32,
    pub compressed: bool,
    pub dynamic_range: u8,
    pub energy: f32,
    pub interior: bool,
    pub normal_bias: f32,
    pub propagation: f32,
    pub subdiv: u8,
    pub extents: Vec3,
}
/// Component holding Godot ReflectionProbe properties.
#[derive(Component, Clone)]
pub struct ReflectionProbe {
    pub projection_enabled: bool,
    pub cull_mask: i64,
    pub shadows_enabled: bool,
    pub extents: Vec3,
    pub intensity: f32,
    pub interior_ambient: (f32, f32, f32, f32),
    pub interior_ambient_probe_contribution: f32,
    pub interior_ambient_energy: f32,
    pub set_as_interior: bool,
    pub max_distance: f32,
    pub origin_offset: Vec3,
    pub update_mode: u8,
}

pub struct NoData;
pub enum LockedStatus {
    Open,
    Closed,
    None,
}
/// Air lock open request event.
pub struct AirLockCloseRequest {
    pub interacter_option: Option<Entity>,
    pub interacted: Entity,
}
