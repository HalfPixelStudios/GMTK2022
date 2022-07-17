use bevy::{app::*, ecs::prelude::*, core::Name, input::Input, prelude::{KeyCode, info, Assets}};
use kayak_ui::{
    bevy::*,
    core::{
        bind, constructor, render, rsx, styles::*, use_state, widget, Binding, Bound, Color,
        MutableBound, VecTracker, WidgetProps,
    },
    widgets::*,
};

use crate::{game::{GameState, Party}, assetloader::{TroopPrefab, PrefabData}, dice::get_dice_coords, troop::DiceTheme};

pub struct UpgradePlugin;

impl Plugin for UpgradePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_set(SystemSet::on_enter(GameState::SelectUpgrades).with_system(render_ui))
            .add_system_set(SystemSet::on_update(GameState::SelectUpgrades).with_system(input_manager))
            .add_system_set(SystemSet::on_exit(GameState::SelectUpgrades).with_system(destroy_ui));
    }
}

#[derive(Default, Clone, PartialEq)]
struct GlobalInput {
    left: bool,
    right: bool,
    up: bool,
    down: bool,
    space: bool,
}

fn render_ui(mut cmd: Commands) {
    cmd.insert_resource(bind(GlobalInput::default()));

    let context = BevyContext::new(|context| {
        let main_container = Style {
            left: StyleProp::Value(Units::Percentage(30.)),
            top: StyleProp::Value(Units::Percentage(10.)),
            width: StyleProp::Value(Units::Percentage(40.)),
            height: StyleProp::Value(Units::Percentage(60.)),
            background_color: StyleProp::Value(Color {
                r: 0.2,
                g: 0.2,
                b: 0.2,
                a: 1.0,
            }),
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
    
    #[derive(Clone, PartialEq)]
    pub struct Page(usize);

    let (chose_upgrade, set_chose_upgrade, ..) = use_state!(None as Option<i32>);
    let (chose_side, set_chose_side, ..) = use_state!(None as Option<(i32, i32)>);
    let (upgrade_cursor, set_upgrade_cursor, ..) = use_state!(0);
    let (dice_cursor, set_dice_cursor, ..) = use_state!((1,1));
    let (page, set_page, ..) = use_state!(Page(0));

    let input_binding =
        context.query_world::<Res<Binding<GlobalInput>>, _, _>(|input| input.clone());
    context.bind(&input_binding);

    // update state
    if page.0 >= 4 {
        info!("done upgrade"); 
        let mut world = context.get_global_mut::<World>().unwrap();
        let mut game_state = world.get_resource_mut::<State<GameState>>().unwrap();
        game_state.set(GameState::StartLevel).unwrap();
    }
    if chose_upgrade.is_none() {
        if input_binding.get().left {
            set_upgrade_cursor((upgrade_cursor - 1).clamp(0, 2));
        }
        if input_binding.get().right {
            set_upgrade_cursor((upgrade_cursor + 1).clamp(0, 2));
        }
        if input_binding.get().space {
            set_chose_upgrade(Some(upgrade_cursor));
        }
    }
    if chose_upgrade.is_some() && chose_side.is_none() {
        let dir = if input_binding.get().left {
            (-1, 0)
        } else if input_binding.get().right {
            (1, 0)
        } else if input_binding.get().up {
            (0, -1)
        } else if input_binding.get().down {
            (0, 1)
        } else {
            (0, 0)
        };

        let allowed_pos = vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2), (1, 3)];

        let new_pos = (dice_cursor.0 + dir.0, dice_cursor.1 + dir.1);
        if allowed_pos.contains(&new_pos) {
            set_dice_cursor(new_pos);
        }

        if input_binding.get().space {
            set_chose_side(Some(new_pos));
        }
    }
    // go to next page
    if chose_side.is_some() {

        info!("go to next page");
        set_chose_upgrade(None);
        set_chose_side(None);
        set_upgrade_cursor(0);
        set_dice_cursor((1,1));
        set_page(Page(page.0+1));
    }

    // get images
    {
        let world = context.get_global::<World>().unwrap();
        let party = world.get_resource::<Res<Party>>().unwrap();
        let prefab_lib = world.get_resource::<Res<Assets<TroopPrefab>>>().unwrap();
        let troop_data = world.get_resource::<Res<PrefabData>>().unwrap();
        let id = party.troops.get(page.0).unwrap();

        let prefab = prefab_lib.get(troop_data.0.get(id).unwrap()).unwrap();
        // TODO get theme from prefab as well

        let coords = prefab.default_dice.sides.iter().map(|s| get_dice_coords(DiceTheme::Warrior, s.clone()));

    }

    // styles
    let grid_width = 32.;
    let container_rows = Style {
        layout_type: StyleProp::Value(LayoutType::Column),
        width: StyleProp::Value(Units::Pixels(grid_width * 3.)),
        height: StyleProp::Value(Units::Pixels(grid_width * 4.)),
        ..Style::default()
    };

    let upgrade_columns = Style {
        layout_type: StyleProp::Value(LayoutType::Row),
        height: StyleProp::Value(Units::Pixels(grid_width)),
        ..Style::default()
    };

    let grid_item0 = Style {
        background_color: StyleProp::Value(Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.0,
        }),
        width: StyleProp::Value(Units::Pixels(grid_width)),
        height: StyleProp::Value(Units::Pixels(grid_width)),
        ..Style::default()
    };

    rsx! {
        <>
            <Text content={format!("page {}", page.0)}></Text>
            <Element styles={Some(container_rows)}>
                {VecTracker::from((0..=3).map(|y| {
                    let container_columns = Style {
                        layout_type: StyleProp::Value(LayoutType::Row),
                        width: StyleProp::Value(Units::Pixels(grid_width * 3.)),
                        height: StyleProp::Value(Units::Pixels(grid_width)),
                        ..Style::default()
                    };
                    constructor! {
                        <Element styles={Some(container_columns)}>
                            {VecTracker::from((0..=2).map(|x| {
                                let mut styles = Style {
                                    background_color: StyleProp::Value(Color { r: 0.0, g: 0.7, b: 0.7, a: 1.0 }),
                                    width: StyleProp::Value(Units::Pixels(grid_width)),
                                    height: StyleProp::Value(Units::Pixels(grid_width)),
                                    ..Style::default()
                                };
                                if chose_upgrade.is_some() && chose_side.is_none() && (x,y) == dice_cursor {
                                    styles.border_color = StyleProp::Value(Color { r: 1.0, g: 1.0, b: 1.0, a: 1.0 });
                                    styles.border = StyleProp::Value(Edge::all(2.));
                                };
                                constructor! {
                                    <Background styles={Some(styles)}></Background>
                                }
                            }))}
                        </Element>
                    }
                }))}
            </Element>

            <Background styles={Some(grid_item0)}></Background>

            <Element styles={Some(upgrade_columns)}>
                {VecTracker::from((0..=2).map(|i| {
                    let mut styles = Style {
                        background_color: StyleProp::Value(Color { r: 0.0, g: 0.7, b: 0.7, a: 1.0 }),
                        width: StyleProp::Value(Units::Pixels(grid_width)),
                        height: StyleProp::Value(Units::Pixels(grid_width)),
                        ..Style::default()
                    };
                    if chose_upgrade.is_none() && i == upgrade_cursor {
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
    if input.just_pressed(KeyCode::Space) {
        input_state.space = true;
    }
    binding.set(input_state);
}

fn destroy_ui(mut cmd: Commands) {
    cmd.remove_resource::<BevyContext>();
}
