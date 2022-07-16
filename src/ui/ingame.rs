use bevy::{app::*, ecs::prelude::*};
use kayak_ui::{
    bevy::*,
    core::{styles::*, *},
    widgets::*,
};

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(render_ui);
    }
}

fn render_ui(mut cmd: Commands) {

    let context = BevyContext::new(|context| {

        let bg_style = Style {
            background_color: StyleProp::Value(Color { r: 1., g: 0., b: 0., a: 1. }),
            width: StyleProp::Value(Units::Pixels(100.)),
            height: StyleProp::Value(Units::Pixels(100.)),
            ..Default::default()
        };
        render! {
            <kayak_ui::widgets::App>
                <Background styles={Some(bg_style)}></Background>
            </kayak_ui::widgets::App>
        }
    });
    cmd.insert_resource(context);
}

fn destroy_ui(mut cmd: Commands) {
    cmd.remove_resource::<BevyContext>();
}
