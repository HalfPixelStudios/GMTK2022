use bevy::prelude::*;
use bevy_bobs::sfx::AudioPlugin;
use talis::animation::*;
use talis::assetloader::*;
use talis::camera::CameraPlugin;
use talis::dice::DicePlugin;
use talis::game::Game;
use talis::game::GameState;
use talis::game::NextTurnEvent;
use talis::layers::Layers;
use talis::map::MapPlugin;
use talis::ui::UIPlugin;
use talis::{assetloader::*, game::GamePlugin, troop::TroopPlugin};

pub struct RunOnce {
    ran: bool,
}
fn main() {
    let window_descriptor = WindowDescriptor {
        present_mode: bevy::window::PresentMode::Fifo,
        title: "bevy_test".into(),
        ..default()
    };

    let mut app = App::new();

    app.insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.5)))
        .insert_resource(window_descriptor);
    // .add_system(bevy::input::system::exit_on_esc_system)

    app.add_plugins(DefaultPlugins)
        .add_plugin(AssetLoadPlugin)
        .add_plugin(AnimationPlugin)
        .add_plugin(AudioPlugin)
        // .add_system(spawn_devil)
        .insert_resource(RunOnce { ran: false })
        .insert_resource(Layers::new())
        .add_plugin(DicePlugin);

    app.add_plugin(GamePlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(MapPlugin)
        .add_plugin(UIPlugin)
        .add_plugin(TroopPlugin);

    app.add_system(debug);

    app.run();
}

// fn spawn_devil(
//     mut commands: Commands,
//     sheets: Res<AssetSheets>,
//     ani_data: Res<PrefabData>,
//     animations: Res<Assets<AnimationPrefab>>,
//     mut ro: ResMut<RunOnce>,
// ) {
//     if ro.ran {
//         return;
//     }

//     if let Some(a) = animations.get(ani_data.0.get(&"RedDemon.ani".to_string()).unwrap()) {
//         ro.ran = true;

//         commands
//             .spawn_bundle(SpriteSheetBundle {
//                 sprite: TextureAtlasSprite {
//                     index: a.anims.get(&AniState::Idle).unwrap().y as usize,
//                     ..default()
//                 },
//                 transform: Transform::from_scale(Vec3::splat(6.0)),
//                 texture_atlas: sheets.0.get(&"assets".to_string()).unwrap().clone(),
//                 ..default()
//             })
//             .insert(Animation {
//                 timer: Timer::from_seconds(0.1, true),
//                 state: AniState::Idle,
//                 data: a.anims.clone(),
//                 finished: false,
//                 index: -1,
//             });
//     }
// }
fn debug(
    keys: Res<Input<KeyCode>>,
    mut game_state: ResMut<State<GameState>>,
    mut game: ResMut<Game>,
    mut next_turn: EventWriter<NextTurnEvent>,
) {
    if keys.just_pressed(KeyCode::S) {
        info!("pressed: starting round");
        game_state.set(GameState::StartLevel).unwrap();
        game.level = 0;
    }
    if keys.just_pressed(KeyCode::T) {
        info!("hoo");
        game_state.set(GameState::StartRound).unwrap();
    }
    if keys.just_pressed(KeyCode::Q) {
        next_turn.send(NextTurnEvent);
    }
}
