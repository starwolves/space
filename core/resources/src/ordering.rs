use bevy::{ecs::schedule::ScheduleLabel, prelude::SystemSet};

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]

pub enum StartupSet {
    ConsoleCommands,
    MiscResources,
    InitDefaultGridmapData,
    BuildGridmap,
    InitAtmospherics,
    InitEntities,
    ServerIsLive,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]

pub enum MapSet {
    ChangeMode,
    MapInput,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]

pub enum ActionsSet {
    Clear,
    Init,
    Build,
    Approve,
    Action,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]

pub enum CombatSet {
    RegisterAttacks,
    CacheAttack,
    WeaponHandler,
    Query,
    StartApplyDamage,
    FinalizeApplyDamage,
    DamageResults,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]

pub enum UpdateSet {
    ProcessMovementInput,
    DropCurrentItem,
    StandardCharacters,
    TextTreeInputSelection,
    DeconstructCell,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]

pub enum SensingSet {
    VisibleChecker,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]

pub enum BuildingSet {
    RawTriggerBuild,
    TriggerBuild,
    NormalBuild,
}

/// The main schedules, ordered. They get called from Bevy's `FixedUpdate` schedule.
/// Use as little schedules as possible as they are hard sync points and limit parallelization.
#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct First;
#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PreUpdate;
/// For maximum parallelization use this main schedule with `SystemSet`s for ordering as much as possible.
#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Update;
#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PostUpdate;
#[derive(ScheduleLabel, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Fin;
