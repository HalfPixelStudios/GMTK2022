use bevy::prelude::*;

use crate::{
    assetloader::{AssetSheets, DicePrefab, PrefabData, Side, TroopPrefab},
    game::GameState,
};

pub struct DicePlugin;
impl Plugin for DicePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(DiceResult::default())
            .add_system_set(SystemSet::on_enter(GameState::Roll).with_system(spawn_dice))
            .add_system_set(SystemSet::on_update(GameState::Roll).with_system(roll_dice));
    }
}

#[derive(Component)]
pub struct DiceUI {
    rolls: i32,
    left: i32,
    timer: Timer,
}
impl DiceUI {
    pub fn reset(&mut self) {
        self.left = self.rolls;
    }
}
pub struct DiceResult {
    pub result: Side,
    pub sides: [Side; 6],
}
impl Default for DiceResult {
    fn default() -> Self {
        DiceResult {
            result: Side::Blank,
            sides: [
                Side::Blank,
                Side::Blank,
                Side::Blank,
                Side::Blank,
                Side::Blank,
                Side::Blank,
            ],
        }
    }
}
fn spawn_dice(
    mut cmd: Commands,
    sheets: Res<AssetSheets>,
    troop_data: Res<PrefabData>,
    mut troops: Res<Assets<TroopPrefab>>,
    mut dice_result: Res<DiceResult>,
) {
    info!("spawn_dice");

    cmd.spawn_bundle(SpriteSheetBundle {
        sprite: TextureAtlasSprite {
            index: 0,
            ..default()
        },
        texture_atlas: sheets.0.get("dice").unwrap().clone(),
        transform: Transform::from_scale(Vec3::splat(1.0)),
        ..default()
    })
    .insert(DiceUI {
        rolls: 5,
        left: 5,
        timer: Timer::from_seconds(0.5, true),
    });
}

fn roll_dice(
    mut cmd: Commands,
    mut dice_query: Query<(Entity, &mut DiceUI, &mut TextureAtlasSprite)>,
    time: Res<Time>,
    mut game_state: ResMut<State<GameState>>,
) {
    for (entity, mut d, mut sprite) in dice_query.iter_mut() {
        if (d.left == 0) {
            cmd.entity(entity).despawn();
            game_state.set(GameState::EndTurn).unwrap();

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
    Skeleton,
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
            if 1 <= number && number <= 9 {
                number as usize
            } else {
                0
            }
        }
        Side::Ability(ability) => {
            // TODO map all the abilties
            0
        }
    };

    (row, column)
}
