use kayak_ui::{
    bevy::*,
    core::{styles::*, widget, rsx, Color, WidgetProps},
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

    let troop_name = props.troop_name.clone();

    rsx! {
        <Element>
            <Text content={troop_name}></Text>
            <PercentBar percent={0.5}></PercentBar>
            <Text content={format!("{}", props.speed)}></Text>
            <Text content={format!("{}", props.defence)}></Text>
        </Element>
    }
}
