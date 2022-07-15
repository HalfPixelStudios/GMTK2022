use bevy::prelude::*;
use bevy_bobs::{
    component::health::Health,
    prefab::{PrefabId, PrefabLib},
};

use crate::prefab::TroopPrefab;

pub struct SpawnTroopEvent {
    id: PrefabId,
}

/*
pub fn spawn_troop(
    mut cmd: &Commands,
    mut events: EventReader<SpawnTroopEvent>,
    prefab_lib: Res<PrefabLib<TroopPrefab>>,
    asset_sheet: Res<AssetSheet>,
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
                    texture_atlas: asset_sheet.0.clone(),
                    ..default()
                })
                .insert(Health::new(prefab.stats.base_health));
        }
    }
}
*/
