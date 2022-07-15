use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy_asset_ron::*;

const ATLAS_WIDTH: usize = 203;
const ATLAS_HEIGHT: usize = 169;

pub struct AssetSheets(pub Vec<Handle<TextureAtlas>>);

pub struct AssetLoadPlugin;

impl Plugin for AssetLoadPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PreStartup, load_assets);
    }
}

pub fn load_assets(
    mut cmd: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mut sheets = Vec::new();

    let image: Handle<Image> = assets.load("AllAssetsPreview.png");

    let atlas = TextureAtlas::from_grid_with_padding(
        image,
        Vec2::new(16.0, 16.0),
        71,
        19,
        Vec2::splat(0.0),
    );
    let atlas_handle = texture_atlases.add(atlas);
    sheets.push(atlas_handle);

    cmd.insert_resource(AssetSheets(sheets));
}
