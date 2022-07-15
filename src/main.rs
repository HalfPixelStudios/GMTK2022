use bevy::prelude::*;
use GMTK2022::{
    assetloader::*,
    game::{GamePlugin, NextTurnEvent, StartRoundEvent},
    prefab::PrefabPlugin,
    troop::TroopPlugin,
};

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugin(AssetLoadPlugin)
        .add_startup_system(setup);

    app.add_plugin(GamePlugin)
        .add_plugin(PrefabPlugin)
        .add_plugin(TroopPlugin);

    app.add_system(debug);

    app.run();
}

fn setup(mut commands: Commands, sheets: Res<AssetSheets>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn debug(
    keys: Res<Input<KeyCode>>,
    mut start_round_writer: EventWriter<StartRoundEvent>,
    mut next_turn_writer: EventWriter<NextTurnEvent>,
) {
    if keys.just_pressed(KeyCode::S) {
        info!("pressed: starting round");
        start_round_writer.send(StartRoundEvent);
    }
    if keys.just_pressed(KeyCode::T) {
        info!("pressed: next turn");
        next_turn_writer.send(NextTurnEvent);
    }
}
