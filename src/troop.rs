use bevy::prelude::*;
use bevy_bobs::component::health::Health;
use serde::Deserialize;

use crate::{
    animation::*,
    assetloader::{AssetSheets, PrefabData, Side, TroopPrefab},
    game::Game,
};

pub struct SpawnTroopEvent {
    pub id: String,
    pub tag: Tag,
    pub spawn_pos: Vec2,
}
pub struct DespawnTroopEvent {
    pub entity: Entity,
}

#[derive(Component)]
pub struct Troop;

#[derive(Component, Clone, PartialEq, Debug)]
pub enum Tag {
    Player,
    Enemy,
}

#[derive(Deserialize, Clone)]
pub enum Class {
    Warrior,
    Wizard,
    Archer,
    Cleric,
}

#[derive(Component)]
pub struct Dice {
    pub sides: [Side; 6],
}

impl Dice {
    pub fn roll(&self) -> Side {
        use rand::{thread_rng, Rng};

        let face: usize = thread_rng().gen_range(0..6);
        self.sides[face].clone()
    }

    pub fn replace(&mut self, index: usize, side: Side) {
        // TODO not safe
        self.sides[index] = side;
    }

    pub fn modify_number(&mut self, index: usize, modify: i32) {}
}
#[derive(Component)]
pub struct Stats {
    base_health: u32,

    health: u32,
    speed: u32,
    defence: u32,

    // buffs: Vec<>
}

impl Stats {
    pub fn new(health: u32, speed: u32, defence: u32) -> Self {
        Stats {
            base_health: health,

            health,
            speed,
            defence,
        }
    }

    pub fn health(&self) -> u32 {
        self.health
    }
    pub fn take_damage(&mut self, amount: u32) {
        // take damange taking defence into account
        self.health = if amount > self.health {
            0
        } else {
            self.health - amount
        };
    }
    pub fn base_health(&self) -> u32 {
        self.base_health
    }
    pub fn is_dead(&self) -> bool {
        self.health == 0
    }

    pub fn speed(&self) -> u32 {
        self.speed
    }

    pub fn defence(&self) -> u32 {
        self.defence
    }

    pub fn add_buff(&mut self) {}
    pub fn clear_buffs(&mut self) {}
}

pub struct TroopPlugin;

impl Plugin for TroopPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnTroopEvent>()
            .add_event::<DespawnTroopEvent>()
            .add_system(spawn_troop_system)
            .add_system(despawn_troop_system);
    }
}

fn spawn_troop_system(
    mut cmd: Commands,
    mut game: ResMut<Game>, // kind ugly to use this here
    mut events: EventReader<SpawnTroopEvent>,
    troop_data: Res<PrefabData>,
    asset_sheet: Res<AssetSheets>,
    troops: Res<Assets<TroopPrefab>>,
) {
    for SpawnTroopEvent { id, tag, spawn_pos } in events.iter() {
        info!(id);
        let prefab = troops.get(troop_data.0.get(id).unwrap()).unwrap();

        let e = cmd.spawn().id();
        cmd.entity(e)
            .insert(Troop)
            .insert(Name::new(prefab.display_name.clone()))
            .insert_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: prefab.anim.frame_ranges.get(&AniState::Idle).unwrap().y as usize,
                    ..default()
                },
                texture_atlas: asset_sheet.0.get("assets").unwrap().clone(),
                transform: Transform {
                    translation: spawn_pos.extend(0.),
                    ..default()
                },
                ..default()
            })
            .insert(Dice {
                sides: prefab.default_dice.sides.clone(),
            })
            .insert(Stats::new(
                prefab.stats.base_health,
                prefab.stats.base_speed,
                prefab.stats.base_defence,
            ))
            // TODO this is stupid
            .insert(match tag {
                Tag::Player => Tag::Player,
                Tag::Enemy => Tag::Enemy,
            })
            .insert(Animation {
                timer: Timer::from_seconds(prefab.anim.seconds, true),
                state: AniState::Idle,
                data: prefab.anim.frame_ranges.clone(),
                finished: false,
                index: -1,
            });

        // add newly spawned troop to game ref
        match tag {
            Tag::Player => {
                game.party.push(e);
            }
            Tag::Enemy => {
                game.enemies.push(e);
            }
        }
    }
}

fn despawn_troop_system(
    mut writer: EventWriter<DespawnTroopEvent>,
    troop_query: Query<(Entity, &Stats), With<Troop>>,
) {
    for (e, stats) in troop_query.iter() {
        if stats.is_dead() {
            info!("troop has died!");
            writer.send(DespawnTroopEvent { entity: e });
        }
    }
}
