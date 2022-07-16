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

pub struct AssetSheets(pub HashMap<String, Handle<TextureAtlas>>);
#[derive(Debug)]
pub struct AnimationData(pub HashMap<String, Handle<AnimationAsset>>);

pub struct AssetLoadPlugin;

impl Plugin for AssetLoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_assets)
            .add_plugin(RonAssetPlugin::<AnimationAsset>::new(&["ani"]));
    }
}

pub fn load_assets(
    mut cmd: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mut sheets = HashMap::new();

    let image: Handle<Image> = assets.load("AllAssetsPreview.png");

    let atlas = TextureAtlas::from_grid_with_padding(
        image,
        Vec2::new(16.0, 16.0),
        71,
        19,
        Vec2::splat(0.0),
    );
    let atlas_handle = texture_atlases.add(atlas);
    sheets.insert("AllAssetsPreview.png".to_string(), atlas_handle);

    cmd.insert_resource(AssetSheets(sheets));
    let mut ani_data: HashMap<String, Handle<AnimationAsset>> = HashMap::new();
    ani_data.insert("RedDemon".to_string(), assets.load("anis/RedDemon.ani"));
    cmd.insert_resource(AnimationData(ani_data));
}
