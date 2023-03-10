use bevy::prelude::SystemSet;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]

pub enum StartupLabels {
    ConsoleCommands,
    MiscResources,
    InitDefaultGridmapData,
    BuildGridmap,
    InitAtmospherics,
    InitEntities,
    ServerIsLive,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]

pub enum MapLabels {
    ChangeMode,
    MapInput,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]

pub enum ActionsLabels {
    Clear,
    Init,
    Build,
    Approve,
    Action,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]

pub enum CombatLabels {
    RegisterAttacks,
    CacheAttack,
    WeaponHandler,
    Query,
    StartApplyDamage,
    FinalizeApplyDamage,
    DamageResults,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]

pub enum UpdateLabels {
    ProcessMovementInput,
    DropCurrentItem,
    StandardCharacters,
    TextTreeInputSelection,
    DeconstructCell,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]

pub enum PostUpdateLabels {
    EntityUpdate,
    SendEntityUpdates,
    VisibleChecker,
    Net,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]

pub enum BuildingLabels {
    TriggerBuild,
    NormalBuild,
}
