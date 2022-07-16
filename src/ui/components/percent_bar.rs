use kayak_ui::{
    bevy::*,
    core::{rsx, styles::*, widget, Color, WidgetProps},
    widgets::*,
};

#[derive(WidgetProps, Debug, PartialEq, Clone)]
pub struct PercentBarProps {
    pub percent: f32,
    pub width: f32,
    pub height: f32,
    #[prop_field(Styles)]
    pub styles: Option<Style>,
}

impl Default for PercentBarProps {
    fn default() -> Self {
        PercentBarProps {
            percent: 1.,
            width: 100.,
            height: 10.,
            styles: Some(Style::default()),
        }
    }
}

#[widget]
pub fn PercentBar(props: PercentBarProps) {
    let bar_percent = props.percent.clamp(0., 1.);

    let fg_style = Style {
        background_color: StyleProp::Value(Color {
            r: 1.,
            g: 0.,
            b: 0.,
            a: 1.,
        }),
        width: StyleProp::Value(Units::Pixels(props.width * bar_percent)),
        height: StyleProp::Value(Units::Pixels(props.height)),
        ..Default::default()
    };

    let bg_style = Style {
        background_color: StyleProp::Value(Color {
            r: 0.,
            g: 0.,
            b: 0.,
            a: 1.,
        }),
        width: StyleProp::Value(Units::Pixels(props.width * (1. - bar_percent))),
        height: StyleProp::Value(Units::Pixels(props.height)),
        ..Default::default()
    };

    let container_style = Style {
        layout_type: StyleProp::Value(LayoutType::Row),
        ..Default::default()
    };
    let container_style = container_style.with_style(&props.styles);

    rsx! {
        <Element styles={Some(container_style)}>
            <Background styles={Some(fg_style)}></Background>
            <Background styles={Some(bg_style)}></Background>
        </Element>
    }
}
