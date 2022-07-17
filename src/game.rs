use bevy::prelude::*;

use crate::{
    animation::*,
    assetloader::{Side, TroopPrefab},
    dice::{DiceResult, DiceUI},
    troop::{self, DespawnTroopEvent, Dice, SpawnTroopEvent, Stats, Tag, Troop},
};
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    Menu,
    StartLevel,
    InRound,
    StartRound,
    StartTurn,
    Roll,
    EndTurn,
    EndRound,
    EndLevel,
    SelectUpgrades,
}

pub struct NextTurnEvent;
pub struct Game {
    pub level: usize,
    pub party: Vec<Entity>,   // player's troops
    pub enemies: Vec<Entity>, // enemy troops
    pub turn_order: Vec<Entity>,
}

// upgradeable party (reinitialize on every level)
pub struct Party {
    troops: Vec<String>,
}
pub struct Level {
    pub enemies: Vec<String>,
    pub room_center: f32, // pub rewards:
}

pub struct Levels {
    levels: Vec<Level>,
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

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<NextTurnEvent>().add_state(GameState::Menu);
        app.add_system_set(SystemSet::on_enter(GameState::StartLevel).with_system(start_level))
            .add_system_set(SystemSet::on_update(GameState::StartLevel).with_system(troop_walk))
            .add_system_set(SystemSet::on_enter(GameState::StartRound).with_system(start_round))
            .add_system_set(SystemSet::on_enter(GameState::StartTurn).with_system(start_turn))
            .add_system_set(SystemSet::on_enter(GameState::EndTurn).with_system(end_turn))
            .add_system_set(SystemSet::on_enter(GameState::EndRound).with_system(end_round))
            .add_system_set(SystemSet::on_enter(GameState::EndLevel).with_system(end_level));

        app.insert_resource(Game::new())
            .add_startup_system(setup)
            .add_system(remove_stupid_dice);
    }
}

fn setup(mut cmd: Commands) {
    cmd.insert_resource(Party {
        troops: vec![
            "warrior.troop".into(),
            "warrior.troop".into(),
            "warrior.troop".into(),
            "warrior.troop".into(),
        ],
    });
    cmd.insert_resource(Levels {
        levels: vec![Level {
            enemies: vec![
                "orc.troop".into(),
                "orc.troop".into(),
                "orc.troop".into(),
                "orc.troop".into(),
            ],
            room_center: 0.,
        }],
    });
}

fn start_level(
    mut cmd: Commands,
    mut game: ResMut<Game>,
    party_res: Res<Party>,
    levels_res: Res<Levels>,
    mut writer: EventWriter<SpawnTroopEvent>,
    mut troop_query: Query<Entity, With<Troop>>,
) {
    for entity in troop_query.iter() {
        cmd.entity(entity).despawn();
    }

    // repopulate both player and enemy lists
    game.party.clear();
    game.enemies.clear();

    // NOTE: the spawn troop handler will add the new entity to game.party and game.enemies
    // (dont like how the logic is somewhere else)
    for (i, troop) in party_res.troops.iter().enumerate() {
        writer.send(SpawnTroopEvent {
            id: troop.clone(),
            tag: Tag::Player,
            spawn_pos: Vec2::new((i as f32) * 100., 100.),
        });
    }

    let level = levels_res.levels.get(game.level).unwrap();
    for (i, enemy) in level.enemies.iter().enumerate() {
        writer.send(SpawnTroopEvent {
            id: enemy.clone(),
            tag: Tag::Enemy,
            spawn_pos: Vec2::new((i as f32) * 100., -100.),
        });
    }
}

fn start_round(
    mut game_state: ResMut<State<GameState>>,
    mut game: ResMut<Game>,
    troop_query: Query<(Entity, &Stats), With<Troop>>,
) {
    info!("start round");
    game.turn_order.clear();

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

    info!("attack order {:?}", game.turn_order);
    game_state.set(GameState::StartTurn).unwrap();
}
fn troop_walk(
    mut game: ResMut<Game>,
    mut troop_query: Query<(&mut Animation, &mut Transform)>,
    mut levels: Res<Levels>,
    mut game_state: ResMut<State<GameState>>,
) {
    let mut arrived = true;
    let level = levels.levels.get(game.level).unwrap();
    let mut count = 0;
    for (mut anims, mut transform) in troop_query.iter_mut() {
        count += 1;
        anims.state = AniState::Idle;
        let sign_dist = level.room_center - transform.translation.y;
        if (transform.translation.y - level.room_center).abs() > 30. {
            anims.state = AniState::Walk;
            transform.translation.y += sign_dist.signum() * 0.8;
            arrived = false;
        }
    }
    if arrived && count != 0 {
        game_state.set(GameState::StartRound).unwrap();
    }
}
fn remove_stupid_dice(
    game_state: ResMut<State<GameState>>,
    mut cmd: Commands,
    stupid_dice: Query<Entity, With<DiceUI>>,
) {
    if *game_state.current() == GameState::Roll {
        return;
    }
    for d in stupid_dice.iter() {
        cmd.entity(d).despawn();
    }
}

pub fn start_turn(
    mut game_state: ResMut<State<GameState>>,
    mut game: ResMut<Game>,
    mut troop_query: Query<(Entity, &Dice, &mut Stats, &Tag)>,
    mut dice_result: ResMut<DiceResult>,
) {
    info!("start_turn");

    if game.turn_order.len() == 0 {
        game_state.set(GameState::StartRound).unwrap();
        return;
    }
    let next_turn = game.turn_order.last().unwrap();
    info!("{:?}", next_turn);

    // roll dice for troop
    let dice = troop_query.get_component::<Dice>(*next_turn).unwrap();
    dice_result.sides = dice.sides.clone();
    dice_result.result = dice.roll();
    game_state.set(GameState::Roll).unwrap();
}

fn end_round(mut game_state: ResMut<State<GameState>>) {
    info!("end round");
    game_state.set(GameState::StartRound).unwrap();
}
fn end_turn(
    mut game: ResMut<Game>,
    dice_result: Res<DiceResult>,
    mut game_state: ResMut<State<GameState>>,
    mut troop_query: Query<(Entity, &Dice, &mut Stats, &Tag)>,
) {
    info!("end_turn");
    info!("{:?}", game.turn_order);

    if game.turn_order.len() == 0 {
        game_state.set(GameState::EndRound).unwrap();
        return;
    }

    let next_turn = game.turn_order.pop().unwrap();

    let tag = troop_query.get_component::<Tag>(next_turn).unwrap();
    match dice_result.result.clone() {
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

    game_state.set(GameState::StartTurn).unwrap();
}

fn end_level(game: Res<Game>, mut game_state: ResMut<State<GameState>>) {
    info!("end_level");
    // for EndLevelEvent { passed } in events.iter() {
    //     println!("level ended!");
    //     if *passed {
    //         writer.send(StartLevelEvent {
    //             level: game.level + 1,
    //         });
    //     } else {
    //         writer.send(StartLevelEvent { level: game.level });
    //     }
    // }
    game_state.set(GameState::SelectUpgrades).unwrap();
}
