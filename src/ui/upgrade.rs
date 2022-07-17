use bevy::{app::*, ecs::prelude::*, core::Name, input::Input, prelude::KeyCode};
use kayak_ui::{
    bevy::*,
    core::{styles::*, Binding, Color, bind, render, rsx, widget, VecTracker, constructor, Bound, MutableBound, WidgetProps, use_state},
    widgets::*,
};

pub struct UpgradePlugin;

impl Plugin for UpgradePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(render_ui).add_system(input_manager);
    }
}


#[derive(Default, Clone, PartialEq)]
struct GlobalInput {
    left: bool,
    right: bool,
    up: bool,
    down: bool,
}

fn render_ui(mut cmd: Commands) {

    cmd.insert_resource(bind(GlobalInput::default()));

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
    
    let (chose_upgrade, set_chose_upgrade, ..) = use_state!(false);
    let (upgrade_cursor, set_upgrade_cursor, ..) = use_state!(0);

    let input_binding = context.query_world::<Res<Binding<GlobalInput>>, _, _>(|input| input.clone());
    context.bind(&input_binding);

    // update state
    if !chose_upgrade {
        if input_binding.get().left {
            set_upgrade_cursor((upgrade_cursor-1).min(0));
        }
        if input_binding.get().right {
            set_upgrade_cursor((upgrade_cursor+1).max(2));
        }
    }

    // styles
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

    let upgrade_columns = Style {
        layout_type: StyleProp::Value(LayoutType::Row),
        height: StyleProp::Value(Units::Pixels(grid_width)),
        ..Style::default()
    };

    let grid_item0 = Style {
        background_color: StyleProp::Value(Color { r: 0.0, g: 0.0, b: 0.0, a: 0.0 }),
        width: StyleProp::Value(Units::Pixels(grid_width)),
        height: StyleProp::Value(Units::Pixels(grid_width)),
        ..Style::default()
    };
    let grid_item1 = Style {
        background_color: StyleProp::Value(Color { r: 0.0, g: 0.7, b: 0.7, a: 1.0 }),
        width: StyleProp::Value(Units::Pixels(grid_width)),
        height: StyleProp::Value(Units::Pixels(grid_width)),
        ..Style::default()
    };
    let grid_item2 = Style {
        background_color: StyleProp::Value(Color { r: 0.7, g: 0.7, b: 0.0, a: 1.0 }),
        width: StyleProp::Value(Units::Pixels(grid_width)),
        height: StyleProp::Value(Units::Pixels(grid_width)),
        ..Style::default()
    };

    rsx! {
        <>
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

            <Background styles={Some(grid_item0)}></Background>

            <Element styles={Some(upgrade_columns)}>
                {VecTracker::from((0..=2).map(|i| {
                    let mut styles = grid_item1.clone();
                    if !chose_upgrade && i == upgrade_cursor {
                        styles.border_color = StyleProp::Value(Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 });
                        styles.border = StyleProp::Value(Edge::all(2.));
                    };

                    constructor! {
                        <Background styles={Some(styles)}></Background>
                    }
                }))}
            </Element>
        </>
    }
}

fn input_manager(input: Res<Input<KeyCode>>, binding: Res<Binding<GlobalInput>>) {

    let mut input_state = GlobalInput::default();
    if input.just_pressed(KeyCode::Left) {
        input_state.left = true;
    }
    if input.just_pressed(KeyCode::Right) {
        input_state.right = true;
    }
    if input.just_pressed(KeyCode::Up) {
        input_state.up = true;
    }
    if input.just_pressed(KeyCode::Down) {
        input_state.down = true;
    }
    binding.set(input_state);

}
