use bevy::prelude::*;
use bevy_bobs::prefab::PrefabId;

use crate::{
    prefab::Side,
    troop::{Dice, SpawnTroopEvent, Stats, Tag, Troop},
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
        app.add_event::<StartLevelEvent>()
            .add_event::<StartRoundEvent>()
            .add_event::<NextTurnEvent>()
            .add_event::<EndRoundEvent>()
            .insert_resource(Game::new())
            .add_startup_system(setup)
            .add_system(start_level)
            .add_system(start_round)
            .add_system(turn_resolver);
    }
}

fn setup(mut spawn_troop_writer: EventWriter<SpawnTroopEvent>) {
    spawn_troop_writer.send(SpawnTroopEvent {
        id: "warrior".into(),
        tag: Tag::Player,
        spawn_pos: Vec2::ZERO,
    });
    spawn_troop_writer.send(SpawnTroopEvent {
        id: "orc".into(),
        tag: Tag::Enemy,
        spawn_pos: Vec2::new(100., 100.),
    });
}

fn start_level(mut game: ResMut<Game>, mut events: EventReader<StartLevelEvent>) {
    for _ in events.iter() {
        game.level += 1;
    }
}

fn start_round(
    mut events: EventReader<StartRoundEvent>,
    mut game: ResMut<Game>,
    troop_query: Query<(Entity, &Stats), With<Troop>>,
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

        println!("attack order {:?}", game.turn_order);
    }
}

fn turn_resolver(
    mut writer: EventWriter<EndRoundEvent>,
    mut events: EventReader<NextTurnEvent>,
    mut game: ResMut<Game>,
    mut troop_query: Query<(Entity, &Dice, &mut Stats, &Tag)>,
) {
    for _ in events.iter() {
        if game.turn_order.len() == 0 {
            writer.send(EndRoundEvent);
            continue;
        }

        let next_turn = game.turn_order.pop().unwrap();

        // roll dice for troop
        let dice = troop_query.get_component::<Dice>(next_turn).unwrap();
        let tag = troop_query.get_component::<Tag>(next_turn).unwrap();
        match dice.roll() {
            Side::Blank => {
                info!("rolled a blank");
            }
            Side::Number(num) => {
                info!("rolled a number: {}", num);

                use rand::{seq::SliceRandom, thread_rng};

                // choose target to attack
                let enemy_pool = match tag {
                    Tag::Player => &game.enemies,
                    Tag::Enemy => &game.party,
                };

                if let Some(target) = enemy_pool.choose(&mut thread_rng()) {
                    let mut target_stat = troop_query
                        .get_component_mut::<Stats>(target.clone())
                        .unwrap();
                    target_stat.take_damage(num);
                }
            }
            Side::Ability(ability) => {
                info!("rolled an ability {}", ability);
            }
        }
    }
}
