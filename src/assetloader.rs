use crate::prefab::Side;

use super::animation::*;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy_asset_ron::*;
use serde::*;
use std::collections::HashMap;

#[derive(Deserialize, TypeUuid, Debug)]
#[uuid = "b7f64775-6e72-4080-9ced-167607f1f0b2"]
pub struct AnimationAsset {
    pub anims: HashMap<AniState, Vec2>,
    pub seconds: i32,
}

#[derive(Deserialize, TypeUuid, Debug)]
#[uuid = "b2943c90-a830-4807-9f8f-5a1c3efe1bd9"]
pub struct DiceAsset {
    pub sheet: String,
    pub sides: [Side; 6],
    pub frames: [i32; 6],
}

pub struct AssetSheets(pub HashMap<String, Handle<TextureAtlas>>);

#[derive(Debug)]
pub struct PrefabData(pub HashMap<String, HandleUntyped>);

pub struct AssetLoadPlugin;

impl Plugin for AssetLoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_assets)
            .add_plugin(RonAssetPlugin::<AnimationAsset>::new(&["ani"]))
            .add_plugin(RonAssetPlugin::<DiceAsset>::new(&["dice"]));
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

    let mut ani_data: HashMap<String, HandleUntyped> = HashMap::new();
    let red_demon: Handle<AnimationAsset> = assets.load("anis/RedDemon.ani");
    ani_data.insert("RedDemon.ani".to_string(), red_demon.clone_untyped());
    cmd.insert_resource(PrefabData(ani_data));
}
