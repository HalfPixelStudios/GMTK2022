use bevy::prelude::*;
use bevy_bobs::{
    component::health::Health,
    prefab::{PrefabId, PrefabLib},
};

use crate::{
    assetloader::AssetSheets,
    game::Game,
    prefab::{DicePrefab, Side, TroopPrefab}, buff::{BuffLifetime, Buff, Stats},
};

pub enum SpawnInfo {
    Id(PrefabId),
    Prefab(TroopPrefab),
}
pub struct SpawnTroopEvent {
    pub spawn_info: SpawnInfo,
    pub tag: Tag,
    pub spawn_pos: Vec2,
}
pub struct DespawnTroopEvent {
    pub entity: Entity,
}

#[derive(Component)]
pub struct Troop;

#[derive(Component)]
pub struct Dice {
    pub sides: [Side; 6],
}

#[derive(Component)]
pub enum Tag {
    Player,
    Enemy,
}

impl Dice {
    pub fn roll(&self) -> Side {
        use rand::{thread_rng, Rng};

        let face = thread_rng().gen_range(0..6);
        self.sides[face].clone()
    }

    pub fn replace(&mut self, index: usize, side: Side) {
        // TODO not safe
        self.sides[index] = side;
    }

    pub fn modify_number(&mut self, index: usize, modify: i32) {}
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
    prefab_lib: Res<PrefabLib<TroopPrefab>>,
    asset_sheet: Res<AssetSheets>,
) {
    for SpawnTroopEvent {
        spawn_info,
        tag,
        spawn_pos,
    } in events.iter()
    {
        let prefab = match spawn_info.clone() {
            SpawnInfo::Id(id) => prefab_lib.get(&id).unwrap(),
            SpawnInfo::Prefab(prefab) => &prefab,
        };

        let e = cmd.spawn().id();
        cmd.entity(e)
            .insert(Troop)
            .insert_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: prefab.sprite_index,
                    ..default()
                },
                texture_atlas: asset_sheet.0.get("AllAssetsPreview.png").unwrap().clone(),
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

