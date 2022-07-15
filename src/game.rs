use bevy::prelude::*;
use bevy_bobs::prefab::PrefabId;

use crate::troop::Stats;

pub struct StartLevelEvent;
pub struct ExecuteTurnEvent;

pub struct Game {
    pub level: usize,
    pub party: Vec<Entity>,   // player's troops
    pub enemies: Vec<Entity>, // enemy troops
}

pub struct Level {
    pub enemies: Vec<PrefabId>,
    // pub rewards:
}

fn start_level(mut cmd: Commands, mut events: EventReader<StartLevelEvent>) {
    for event in events.iter() {}
}

fn turn_resolver(game: Res<Game>, troop_query: Query<(Entity, &Stats)>) {
    // determine attacking order based on speed
    for (e, stats) in troop_query.iter() {
        // check if troop should actually be in level (may or may not be needed)
        if !game.party.contains(&e) && !game.enemies.contains(&e) {
            warn!("found troop that is not in level");
            continue;
        }
    }

    // roll dice and execute ability
}
