use bevy::prelude::*;
use bevy_bobs::prefab::PrefabId;

pub struct SpawnEnemyEvent {
    id: PrefabId,
}
