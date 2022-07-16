use kayak_ui::{
    bevy::*,
    core::{styles::*, widget, rsx, Color, WidgetProps},
    widgets::*,
};

#[derive(WidgetProps, Debug, PartialEq, Clone)]
pub struct PercentBarProps {
    pub percent: f32,
    pub width: f32,
    pub height: f32,
}

impl Default for PercentBarProps {
    fn default() -> Self {
        PercentBarProps {
            percent: 1.,
            width: 100.,
            height: 10.,
        }
    }
}

#[widget]
pub fn PercentBar(props: PercentBarProps) {

    let bar_percent = props.percent.clamp(0., 1.);

    let fg_style = Style {
        background_color: StyleProp::Value(Color { r: 1., g: 0., b: 0., a: 1. }),
        width: StyleProp::Value(Units::Pixels(props.width * bar_percent)),
        height: StyleProp::Value(Units::Pixels(props.height)),
        ..Default::default()
    };

    let bg_style = Style {
        background_color: StyleProp::Value(Color { r: 0., g: 0., b: 0., a: 1. }),
        width: StyleProp::Value(Units::Pixels(props.width * (1. - bar_percent))),
        height: StyleProp::Value(Units::Pixels(props.height)),
        ..Default::default()
    };
    
    let container_style = Style {
        layout_type: StyleProp::Value(LayoutType::Row),
        ..Default::default()
    };

    rsx! {
        <Element styles={Some(container_style)}>
            <Background styles={Some(fg_style)}></Background>
            <Background styles={Some(bg_style)}></Background>
        </Element>
    }
}

