use bevy::{app::*, ecs::prelude::*, core::Name};
use kayak_ui::{
    bevy::*,
    core::{styles::*, Binding, Color, bind, render, rsx, widget, VecTracker, constructor, Bound, MutableBound, WidgetProps},
    widgets::*,
};

pub struct UpgradePlugin;

impl Plugin for UpgradePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(render_ui);        
    }
}

fn render_ui(mut cmd: Commands) {

    let context = BevyContext::new(|context| {

        let main_container = Style {
            left: StyleProp::Value(Units::Percentage(30.)),
            top: StyleProp::Value(Units::Percentage(10.)),
            width: StyleProp::Value(Units::Percentage(40.)),
            height: StyleProp::Value(Units::Percentage(80.)),
            background_color: StyleProp::Value(Color { r: 0.2, g: 0.2, b: 0.2, a: 1.0 }),
            ..Style::default()
        };

        render! {
            <kayak_ui::widgets::App>
                <Background styles={Some(main_container)}></Background>
            </kayak_ui::widgets::App>
        }
    });
    cmd.insert_resource(context);
}

