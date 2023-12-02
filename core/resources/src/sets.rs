use bevy::prelude::SystemSet;

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

pub enum PostUpdateSet {
    VisibleChecker,
    Net,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]

pub enum BuildingSet {
    RawTriggerBuild,
    TriggerBuild,
    NormalBuild,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]

pub enum MainSet {
    PreUpdate,
    Update,
    PostUpdate,
    PostPhysics,
}
