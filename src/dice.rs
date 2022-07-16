use bevy::prelude::*;
use bevy_bobs::prefab::PrefabId;

use crate::assetloader::{AssetSheets, PrefabData};

pub struct DicePlugin;
impl Plugin for DicePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RollDiceEvent>()
            .add_startup_system(spawn_dice);
    }
}
#[derive(Component)]
struct DiceRoll {
    rolls: i32,
    left: i32,
    timer: Timer,
}
impl DiceRoll {
    pub fn reset(&mut self) {
        self.left = self.rolls;
    }
}
pub struct RollDiceEvent {
    id: String,
}

fn spawn_dice(
    mut cmd: Commands,
    sheets: Res<AssetSheets>,
    mut events: EventReader<RollDiceEvent>,
    dice_data: Res<PrefabData>,
) {
    for e in events.iter() {
        cmd.spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 0,
                ..default()
            },
            texture_atlas: sheets.0.get(&"dice".to_string()).unwrap().clone(),
            ..default()
        })
        .insert(DiceRoll {
            rolls: 5,
            left: 5,
            timer: Timer::from_seconds(0.1, true),
        });
    }
}
fn roll_dice(
    mut cmd: Commands,
    mut dice_query: Query<(Entity, &mut DiceRoll, &mut TextureAtlasSprite)>,
    time: Res<Time>,
) {
    for (entity, mut d, mut sprite) in dice_query.iter_mut() {
        if (d.left == 0) {
            cmd.entity(entity).despawn();
            return;
        }

        d.timer.tick(time.delta());

        if d.timer.just_finished() {
            d.left -= 1;
            sprite.index += 1;
        }
    }
}
