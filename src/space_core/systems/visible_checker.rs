use bevy::prelude::Query;

use crate::space_core::components::{visible::Visible, visible_checker::VisibleChecker};

pub fn visible_checker(
    mut query_visible_entities : Query<&mut Visible>,
    query_visible_checker_entities : Query<&VisibleChecker>,

) {

    // Loop through all relevant entities and automatically obtain their transform translations.
    // Checkers can "cached see" and "normal see" entities.
    // Checkers should store who they already (cached) see.
    // Checkers should now be able to see omni_lights, other players AND themselves with extra logic.
    // When entities are (cached) seen and unseen by checkers, netcode them to appear/disappear/cache/uncache.
    // When we connect as a player we now see the full map in perfect condition, including ourselves.

    

}
