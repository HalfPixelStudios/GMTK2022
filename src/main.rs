use bevy::prelude::*;
use GMTK2022::animation::*;
use GMTK2022::assetloader::*;
use GMTK2022::{
    assetloader::*,
    game::{GamePlugin, NextTurnEvent, StartLevelEvent, StartRoundEvent},
    prefab::PrefabPlugin,
    troop::TroopPlugin,
};

pub struct RunOnce {
    ran: bool,
}
fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugin(AssetLoadPlugin)
        .add_plugin(AnimationPlugin)
        .add_system(setup)
        .add_system(spawn_devil)
        .insert_resource(RunOnce { ran: false })
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

fn spawn_devil(
    mut commands: Commands,
    sheets: Res<AssetSheets>,
    ani_data: Res<AnimationData>,
    animations: Res<Assets<AnimationAsset>>,
    mut ro: ResMut<RunOnce>,
) {
    if ro.ran {
        return;
    }

    if let Some(a) = animations.get(ani_data.0.get(&"RedDemon".to_string()).unwrap()) {
        ro.ran = true;

        commands
            .spawn_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite { ..default() },
                transform: Transform::from_scale(Vec3::splat(6.0)),
                texture_atlas: sheets
                    .0
                    .get(&"AllAssetsPreview.png".to_string())
                    .unwrap()
                    .clone(),
                ..default()
            })
            .insert(Animation {
                timer: Timer::from_seconds(0.1, true),
                state: AniState::Idle,
                data: a.anims.clone(),
                finished: false,
                index: -1,
            });
=======
fn debug(
    keys: Res<Input<KeyCode>>,
    mut start_round_writer: EventWriter<StartLevelEvent>,
    mut next_turn_writer: EventWriter<NextTurnEvent>,
) {
    if keys.just_pressed(KeyCode::S) {
        info!("pressed: starting round");
        start_round_writer.send(StartLevelEvent { level: 0 });
    }
    if keys.just_pressed(KeyCode::T) {
        info!("pressed: next turn");
        next_turn_writer.send(NextTurnEvent);
>>>>>>> master
    }
}
