use super::animation::*;
use crate::troop::*;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy_asset_ron::*;
use serde::*;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone)]
pub struct AnimationPrefab {
    pub frame_ranges: HashMap<AniState, Vec2>,
    pub seconds: f32,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DicePrefab {
    pub sides: [Side; 6],
}
#[derive(Deserialize, Clone)]
pub struct StatsPrefab {
    pub base_health: u32,
    pub base_speed: u32,
    pub base_defence: u32,
}

#[derive(Debug, Deserialize, Clone)]
pub enum Side {
    Blank,
    Number(u32),
    Ability(String),
}

#[derive(Deserialize, TypeUuid)]
#[uuid = "e60395c0-f873-41dc-adfa-42d3ca74b8fc"]
pub struct TroopPrefab {
    pub display_name: String,
    pub stats: StatsPrefab,
    pub default_dice: DicePrefab,
    pub anim: AnimationPrefab,
    pub class: Class,
}

pub struct AssetSheets(pub HashMap<String, Handle<TextureAtlas>>);

#[derive(Debug)]
pub struct PrefabData(pub HashMap<String, HandleUntyped>);

pub struct AssetLoadPlugin;

impl Plugin for AssetLoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_assets)
            .add_plugin(RonAssetPlugin::<TroopPrefab>::new(&["troop"]));
    }
}

pub fn load_assets(
    mut cmd: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mut sheets = HashMap::new();

    let assets_handle = texture_atlases.add(TextureAtlas::from_grid_with_padding(
        assets.load("AllAssetsPreview.png"),
        Vec2::new(16.0, 16.0),
        71,
        19,
        Vec2::splat(0.0),
    ));
    let dice_handle = texture_atlases.add(TextureAtlas::from_grid_with_padding(
        assets.load("diceRed.png"),
        Vec2::new(64., 64.),
        4,
        2,
        Vec2::splat(0.0),
    ));
    sheets.insert("assets".to_string(), assets_handle);
    sheets.insert("dice".to_string(), dice_handle);

    cmd.insert_resource(AssetSheets(sheets));

    let mut data: HashMap<String, HandleUntyped> = HashMap::new();
    let red_demon: Handle<TroopPrefab> = assets.load("troops/orc.troop");
    data.insert("orc.troop".to_string(), red_demon.clone_untyped());
    let warrior: Handle<TroopPrefab> = assets.load("troops/war.troop");
    data.insert("warrior.troop".to_string(), warrior.clone_untyped());
    cmd.insert_resource(PrefabData(data));
}
