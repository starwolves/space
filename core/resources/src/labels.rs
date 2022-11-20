use bevy::prelude::SystemLabel;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
#[cfg(feature = "server")]
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
#[cfg(feature = "server")]
pub enum MapLabels {
    ChangeMode,
    MapInput,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
#[cfg(feature = "server")]
pub enum ActionsLabels {
    Clear,
    Init,
    Build,
    Approve,
    Action,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
#[cfg(feature = "server")]
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
#[cfg(feature = "server")]
pub enum UpdateLabels {
    ProcessMovementInput,
    DropCurrentItem,
    StandardCharacters,
    TextTreeInputSelection,
    DeconstructCell,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
#[cfg(feature = "server")]
pub enum PostUpdateLabels {
    EntityUpdate,
    SendEntityUpdates,
    VisibleChecker,
    Net,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
#[cfg(feature = "server")]
pub enum SummoningLabels {
    TriggerSummon,
    DefaultSummon,
    NormalSummon,
}
