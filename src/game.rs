use bevy::prelude::*;
use bevy_bobs::prefab::PrefabId;

use crate::{
    prefab::Side,
    troop::{Dice, Stats},
};

pub struct StartLevelEvent;
pub struct StartRoundEvent;
pub struct NextTurnEvent;
pub struct EndRoundEvent;

pub struct Game {
    pub level: usize,
    pub party: Vec<Entity>,   // player's troops
    pub enemies: Vec<Entity>, // enemy troops
    pub turn_order: Vec<Entity>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            level: 0,
            party: vec![],
            enemies: vec![],
            turn_order: vec![],
        }
    }
}

pub struct Level {
    pub enemies: Vec<PrefabId>,
    // pub rewards:
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<StartRoundEvent>()
            .add_event::<NextTurnEvent>()
            .add_event::<EndRoundEvent>()
            .add_system(start_round)
            .insert_resource(Game::new())
            .add_system(turn_resolver);
    }
}

fn start_round(
    mut events: EventReader<StartRoundEvent>,
    mut game: ResMut<Game>,
    troop_query: Query<(Entity, &Stats)>,
) {
    for _ in events.iter() {
        // determine attacking order based on speed
        // all this code could be simplified
        let mut attack_order: Vec<(Entity, &Stats)> = vec![];
        for (e, stats) in troop_query.iter() {
            // check if troop should actually be in level (may or may not be needed)
            if !game.party.contains(&e) && !game.enemies.contains(&e) {
                warn!("found troop that is not in level");
                continue;
            }

            attack_order.push((e, stats));
        }
        attack_order.sort_by_key(|k| k.1.speed());

        game.turn_order = attack_order
            .iter()
            .map(|(e, _)| e.clone())
            .collect::<Vec<Entity>>();
    }
}

fn turn_resolver(
    mut writer: EventWriter<EndRoundEvent>,
    mut events: EventReader<NextTurnEvent>,
    mut game: ResMut<Game>,
    troop_query: Query<(Entity, &Dice, &Stats)>,
) {
    for _ in events.iter() {
        if game.turn_order.len() == 0 {
            writer.send(EndRoundEvent);
            continue;
        }

        let next_turn = game.turn_order.pop().unwrap();

        // roll dice for troop
        let dice = troop_query.get_component::<Dice>(next_turn).unwrap();
        match dice.roll() {
            Side::Blank => {}
            Side::Number(num) => {
                // choose target to attack
            }
            Side::Ability(ability) => {}
        }
    }
}
