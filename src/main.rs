use bevy::prelude::*;
use GMTK2022::assetloader::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(AssetLoadPlugin)
        .add_startup_system(setup)
        .run();
}

fn setup(mut commands: Commands, sheets: Res<AssetSheets>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(SpriteSheetBundle {
        sprite: TextureAtlasSprite { ..default() },
        texture_atlas: sheets.0.get(0).unwrap().clone(),
        transform: Transform { ..default() },
        ..default()
    });
}
