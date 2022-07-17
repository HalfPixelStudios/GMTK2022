use bevy::prelude::*;

use crate::assetloader::{AssetSheets, DicePrefab, PrefabData, TroopPrefab, Side};

pub struct DicePlugin;
impl Plugin for DicePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RollDiceEvent>()
            .add_system(spawn_dice)
            .add_system(roll_dice);
    }
}
#[derive(Component)]
struct DiceUI {
    rolls: i32,
    left: i32,
    timer: Timer,
}
impl DiceUI {
    pub fn reset(&mut self) {
        self.left = self.rolls;
    }
}
pub struct RollDiceEvent {
    pub id: String,
}

fn spawn_dice(
    mut cmd: Commands,
    sheets: Res<AssetSheets>,
    mut events: EventReader<RollDiceEvent>,
    troop_data: Res<PrefabData>,
    mut troops: Res<Assets<TroopPrefab>>,
) {
    for e in events.iter() {
        info!("{:?}", e.id);

        let dice = troops
            .get(troop_data.0.get(&e.id).unwrap())
            .unwrap()
            .default_dice
            .clone();

        cmd.spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 0,
                ..default()
            },
            texture_atlas: sheets.0.get("dice").unwrap().clone(),
            transform: Transform::from_scale(Vec3::splat(6.0)),
            ..default()
        })
        .insert(DiceUI {
            rolls: 5,
            left: 5,
            timer: Timer::from_seconds(0.1, true),
        });
    }
}

fn roll_dice(
    mut cmd: Commands,
    mut dice_query: Query<(Entity, &mut DiceUI, &mut TextureAtlasSprite)>,
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

pub enum DiceTheme {
    Warrior,
    Cleric,
    Archer,
    Mage,

    GreenSlime,
    BlueSlime,
    Orc,
    Crab,
    Skeleton
}

pub fn get_dice_coords(theme: DiceTheme, side: Side) -> (usize, usize) {

    let row: usize = match theme {
        DiceTheme::Warrior => 0,
        DiceTheme::Cleric => 1,
        DiceTheme::Archer => 2,
        DiceTheme::Mage => 3,
        DiceTheme::GreenSlime => 4,
        DiceTheme::BlueSlime => 5,
        DiceTheme::Orc => 6,
        DiceTheme::Crab => 7,
        DiceTheme::Skeleton => 8,
    };

    let column: usize = match side {
        Side::Blank => 0, 
        Side::Number(number) => {
            if 1 <= number && number <= 9 { number as usize } else { 0 }
        },
        Side::Ability(ability) => {
            // TODO map all the abilties
            0
        }
    };

    (row, column)
}

