pub mod components;
pub mod ingame;

use bevy::prelude::*;
use kayak_ui::bevy::BevyKayakUIPlugin;

use self::ingame::InGamePlugin;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(BevyKayakUIPlugin)
            .add_plugin(InGamePlugin)
            .add_startup_system(setup);
    }
}

fn setup(mut cmd: Commands) {
    cmd.spawn_bundle(kayak_ui::bevy::UICameraBundle::new());
}
