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
            height: StyleProp::Value(Units::Percentage(60.)),
            background_color: StyleProp::Value(Color { r: 0.2, g: 0.2, b: 0.2, a: 1.0 }),
            padding: StyleProp::Value(Edge::all(Units::Percentage(10.))),
            ..Style::default()
        };

        render! {
            <kayak_ui::widgets::App>
                <Background styles={Some(main_container)}>
                    <Text content={"upgrade".into()}></Text>
                    <UpgradeMenu></UpgradeMenu>
                </Background>
            </kayak_ui::widgets::App>
        }
    });
    cmd.insert_resource(context);
}

#[widget]
fn UpgradeMenu() {

    let grid_width = 32.;
    let container_rows = Style {
        layout_type: StyleProp::Value(LayoutType::Column),
        width: StyleProp::Value(Units::Pixels(grid_width * 3.)),
        height: StyleProp::Value(Units::Pixels(grid_width * 4.)),
        ..Style::default()
    };
    let container_columns = Style {
        layout_type: StyleProp::Value(LayoutType::Row),
        width: StyleProp::Value(Units::Pixels(grid_width * 3.)),
        height: StyleProp::Value(Units::Pixels(grid_width)),
        ..Style::default()
    };

    let grid_item1 = Style {
        background_color: StyleProp::Value(Color { r: 0.0, g: 0.7, b: 0.7, a: 1.0 }),
        ..Style::default()
    };
    let grid_item2 = Style {
        background_color: StyleProp::Value(Color { r: 0.7, g: 0.7, b: 0.0, a: 1.0 }),
        ..Style::default()
    };

    rsx! {
        <Element styles={Some(container_rows)}>
            <Element styles={Some(container_columns)}>
                <Background styles={Some(grid_item1)}></Background>
                <Background styles={Some(grid_item2)}></Background>
                <Background styles={Some(grid_item1)}></Background>
            </Element>
            <Element styles={Some(container_columns)}>
                <Background styles={Some(grid_item2)}></Background>
                <Background styles={Some(grid_item1)}></Background>
                <Background styles={Some(grid_item2)}></Background>
            </Element>
            <Element styles={Some(container_columns)}>
                <Background styles={Some(grid_item1)}></Background>
                <Background styles={Some(grid_item2)}></Background>
                <Background styles={Some(grid_item1)}></Background>
            </Element>
            <Element styles={Some(container_columns)}>
                <Background styles={Some(grid_item2)}></Background>
                <Background styles={Some(grid_item1)}></Background>
                <Background styles={Some(grid_item2)}></Background>
            </Element>
        </Element>
    }
}
