use std::collections::HashSet;

use bevy::prelude::*;

use super::{peasant::Peasant, PEASANT_MAX_HEALTH, TOTAL_KILL_REQUIRED_TO_BEAT_LEVEL};

pub struct PeasantKilled(pub Entity);

#[derive(Resource, Default)]
pub struct TotalPeasantsKilled(pub HashSet<Entity>);

pub fn track_kills(
    mut peasant_killed_event: EventReader<PeasantKilled>,
    mut total_peasants_killed: ResMut<TotalPeasantsKilled>,
) {
    for e in peasant_killed_event.iter() {
        total_peasants_killed.0.insert(e.0);
    }

    if total_peasants_killed.0.len() >= TOTAL_KILL_REQUIRED_TO_BEAT_LEVEL {
        // Do a thing
    }
}
