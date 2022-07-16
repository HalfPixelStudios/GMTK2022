use bevy::{app::*, ecs::prelude::*};
use kayak_ui::{
    bevy::*,
    core::{styles::*, *},
    widgets::*,
};

use super::components::health_bar::HealthBar;

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(render_ui);
    }
}

fn render_ui(mut cmd: Commands) {

    let context = BevyContext::new(|context| {

        render! {
            <kayak_ui::widgets::App>
                <HealthBar percent={0.5}></HealthBar>
            </kayak_ui::widgets::App>
        }
    });
    cmd.insert_resource(context);
}

fn destroy_ui(mut cmd: Commands) {
    cmd.remove_resource::<BevyContext>();
}
