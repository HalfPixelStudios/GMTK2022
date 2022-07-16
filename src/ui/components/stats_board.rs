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

    let image_dimensions = Vec2::new(16., 16.);

    let container_style = Style {
        layout_type: StyleProp::Value(LayoutType::Column),
        height: StyleProp::Value(Units::Pixels(200.)),
        ..Style::default()
    };

    let text_style = Style {
        line_height: StyleProp::Value(image_dimensions.x),
        ..Style::default()
    };
    let image_style = Style {
        width: StyleProp::Value(Units::Pixels(image_dimensions.x)),
        height: StyleProp::Value(Units::Pixels(image_dimensions.y)),
        ..Style::default()
    };
    let stat_container_style = Style {
        layout_type: StyleProp::Value(LayoutType::Row),
        // col_between: StyleProp::Value(Units::Pixels(image_padding)),
        ..Style::default()
    };
    let hp_bar_style = Style {
        left: StyleProp::Value(Units::Pixels(10.)),
        top: StyleProp::Value(Units::Pixels(17.)),
        ..Style::default()
    };

    rsx! {
        <Element styles={Some(container_style)}>
            <Text content={troop_name} styles={Some(text_style)}></Text>
            <Element styles={Some(stat_container_style.clone())}>
                <Image handle={heart_handle} styles={Some(image_style)}></Image>
                <PercentBar percent={props.health_percent} styles={Some(hp_bar_style)}></PercentBar>
            </Element>
            <Element styles={Some(stat_container_style.clone())}>
                <Image handle={speed_handle} styles={Some(image_style)}></Image>
                <Text styles={Some(text_style)} content={format!("{}", props.speed)}></Text>
            </Element>
            <Element styles={Some(stat_container_style.clone())}>
                <Image handle={defence_handle} styles={Some(image_style)}></Image>
                <Text styles={Some(text_style)} content={format!("{}", props.defence)}></Text>
            </Element>
        </Element>
    }
}
