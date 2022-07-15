use bevy::prelude::*;
use bevy_bobs::{
    component::health::Health,
    prefab::{PrefabId, PrefabLib},
};

use crate::{
    assetloader::AssetSheets,
    prefab::{DicePrefab, Side, TroopPrefab},
};

pub struct SpawnTroopEvent {
    id: PrefabId,
}

#[derive(Component)]
pub struct Stats {
    health: u32,
    speed: u32,
    defence: u32,
    // buffs: Vec<>
}

#[derive(Component)]
pub struct Dice {
    pub sides: [Side; 6],
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

impl Stats {
    pub fn new(health: u32, speed: u32, defence: u32) -> Self {
        Stats {
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
        app.add_event::<SpawnTroopEvent>().add_system(spawn_troop);
    }
}

pub fn spawn_troop(
    mut cmd: Commands,
    mut events: EventReader<SpawnTroopEvent>,
    prefab_lib: Res<PrefabLib<TroopPrefab>>,
    asset_sheet: Res<AssetSheets>,
) {
    for SpawnTroopEvent { id } in events.iter() {
        if let Some(prefab) = prefab_lib.get(id) {
            let e = cmd.spawn().id();
            cmd.entity(e)
                .insert_bundle(SpriteSheetBundle {
                    sprite: TextureAtlasSprite {
                        index: prefab.sprite_index,
                        ..default()
                    },
                    texture_atlas: asset_sheet.0.get(0).unwrap().clone(),
                    ..default()
                })
                .insert(Dice {
                    sides: prefab.default_dice.sides.clone(),
                })
                .insert(Stats::new(
                    prefab.stats.base_health,
                    prefab.stats.base_speed,
                    prefab.stats.base_defence,
                ));
        }
    }
}
