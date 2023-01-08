use bevy::prelude::SystemLabel;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]

pub enum StartupLabels {
    ConsoleCommands,
    MiscResources,
    InitDefaultGridmapData,
    BuildGridmap,
    InitAtmospherics,
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

pub enum BuildingLabels {
    TriggerBuild,
    NormalBuild,
}
