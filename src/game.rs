use bevy::prelude::*;
use bevy_bobs::prefab::PrefabId;

use crate::{
    prefab::Side,
    troop::{self, DespawnTroopEvent, Dice, SpawnTroopEvent, Stats, Tag, Troop},
};

pub struct StartLevelEvent {
    pub level: usize,
}
pub struct StartRoundEvent;
pub struct NextTurnEvent;
pub struct EndRoundEvent;
pub struct EndLevelEvent {
    pub passed: bool, // did player win or lose
}

pub struct Game {
    pub level: usize,
    pub party: Vec<Entity>,   // player's troops
    pub enemies: Vec<Entity>, // enemy troops
    pub turn_order: Vec<Entity>,
}

pub struct Party {}
pub struct Levels {}

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
            .add_event::<EndLevelEvent>()
            .insert_resource(Game::new())
            .add_startup_system(setup)
            .add_system(start_level)
            .add_system(start_round)
            .add_system(turn_resolver)
            .add_system(end_round)
            .add_system(end_level)
            .add_system(troop_died_event);
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
    for StartLevelEvent { level } in events.iter() {
        game.level = *level;

        // repopulate both player and enemy lists
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
                    println!("target {:?}", target);
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

fn end_round(mut events: EventReader<EndRoundEvent>, mut writer: EventWriter<StartRoundEvent>) {
    for _ in events.iter() {
        writer.send(StartRoundEvent);
    }
}

fn end_level(
    game: Res<Game>,
    mut events: EventReader<EndLevelEvent>,
    mut writer: EventWriter<StartLevelEvent>,
) {
    for EndLevelEvent { passed } in events.iter() {
        println!("level ended!");
        if *passed {
            writer.send(StartLevelEvent {
                level: game.level + 1,
            });
        } else {
            writer.send(StartLevelEvent { level: game.level });
        }
    }
}

fn troop_died_event(
    mut cmd: Commands,
    mut game: ResMut<Game>,
    mut events: EventReader<DespawnTroopEvent>,
    mut writer: EventWriter<EndLevelEvent>,
    troop_query: Query<(Entity, &Tag), With<Troop>>,
) {
    for DespawnTroopEvent { entity } in events.iter() {
        if let Ok(tag) = troop_query.get_component::<Tag>(*entity) {
            match *tag {
                Tag::Player => &mut game.party,
                Tag::Enemy => &mut game.enemies,
            }
            .retain(|e| e != entity);

            cmd.entity(*entity).despawn();

            println!(
                "party left {}, enemies left {}",
                game.party.len(),
                game.enemies.len()
            );
        }

        // check win or lose condition
        if game.party.len() == 0 {
            writer.send(EndLevelEvent { passed: false });
        }
        if game.enemies.len() == 0 {
            writer.send(EndLevelEvent { passed: true });
        }
    }
}
