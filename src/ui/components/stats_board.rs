use bevy::{
    math::Vec2,
    prelude::{AssetServer, World},
};
use kayak_ui::{
    bevy::*,
    core::{rsx, styles::*, widget, Color, WidgetProps},
    widgets::*,
};

use super::percent_bar::PercentBar;

#[derive(WidgetProps, Default, Debug, PartialEq, Clone)]
pub struct StatsBoardProps {
    pub troop_name: String,
    pub health_percent: f32,
    pub speed: u32,
    pub defence: u32,
}

#[widget]
pub fn StatsBoard(props: StatsBoardProps) {
    let (heart_handle, speed_handle, defence_handle) = {
        let mut world = context.get_global_mut::<World>().unwrap();

        let (heart_handle, speed_handle, defence_handle) = {
            let asset_server = world.get_resource::<AssetServer>().unwrap();
            (
                asset_server.load("ui/heart.png"),
                asset_server.load("ui/speed.png"),
                asset_server.load("ui/shield.png"),
            )
        };

        let mut image_manager = world.get_resource_mut::<ImageManager>().unwrap();

        (
            image_manager.get(&heart_handle),
            image_manager.get(&speed_handle),
            image_manager.get(&defence_handle),
        )
    };

    let troop_name = props.troop_name.clone();
    let image_size = 16.;

    let container_style = Style {
        layout_type: StyleProp::Value(LayoutType::Column),
        height: StyleProp::Value(Units::Pixels(80.)),
        ..Style::default()
    };

    let alpha = 0.5;
    let health_bar_box = Style {
        background_color: StyleProp::Value(Color {
            r: 0.,
            g: 1.,
            b: 0.,
            a: alpha,
        }),
        layout_type: StyleProp::Value(LayoutType::Row),
        ..Style::default()
    };
    let speed_box = Style {
        background_color: StyleProp::Value(Color {
            r: 0.,
            g: 1.,
            b: 1.,
            a: alpha,
        }),
        layout_type: StyleProp::Value(LayoutType::Row),
        ..Style::default()
    };
    let defence_box = Style {
        background_color: StyleProp::Value(Color {
            r: 1.,
            g: 1.,
            b: 0.,
            a: alpha,
        }),
        layout_type: StyleProp::Value(LayoutType::Row),
        ..Style::default()
    };

    let image_style = Style {
        width: StyleProp::Value(Units::Pixels(image_size)),
        height: StyleProp::Value(Units::Pixels(image_size)),
        ..Style::default()
    };
    let health_bar_style = Style {
        top: StyleProp::Value(Units::Pixels(5.)),
        ..Style::default()
    };

    rsx! {
        <Element styles={Some(container_style)}>
            <Text content={troop_name}></Text>
            <Background styles={Some(health_bar_box)}>
                <Image handle={heart_handle} styles={Some(image_style)}></Image>
                <PercentBar percent={props.health_percent} styles={Some(health_bar_style)}></PercentBar>
            </Background>
            <Background styles={Some(speed_box)}>
                <Image handle={speed_handle} styles={Some(image_style)}></Image>
                <Text content={format!("{}", props.speed)}></Text>
            </Background>
            <Background styles={Some(defence_box)}>
                <Image handle={defence_handle} styles={Some(image_style)}></Image>
                <Text content={format!("{}", props.defence)}></Text>
            </Background>
        </Element>
    }
}
