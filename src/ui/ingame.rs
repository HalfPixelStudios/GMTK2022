use bevy::{app::*, ecs::prelude::*, core::Name};
use kayak_ui::{
    bevy::*,
    core::{styles::*, Binding, Color, bind, render, rsx, widget, VecTracker, constructor, Bound, MutableBound},
    widgets::*,
};

use crate::{troop::{Tag, Stats}, game::Game};

use super::components::stats_board::StatsBoard;

pub struct InGamePlugin;

impl Plugin for InGamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system(render_ui)
            .add_system(update_stat_data);
    }
}

#[derive(Clone, PartialEq)]
struct StatDataEntry {
    pub troop_name: String,
    pub health_percent: f32,
    pub speed: u32,
    pub defence: u32,
}

#[derive(Default, Clone, PartialEq)]
struct StatData {
    party: Vec<StatDataEntry>,
    enemies: Vec<StatDataEntry>,
}

fn render_ui(mut cmd: Commands) {

    cmd.insert_resource(bind(StatData::default()));

    let context = BevyContext::new(|context| {

        render! {
            <kayak_ui::widgets::App>
                <StatsList></StatsList>
            </kayak_ui::widgets::App>
        }
    });
    cmd.insert_resource(context);
}

#[widget]
fn StatsList() {

    let stat_data_binding = context.query_world::<Res<Binding<StatData>>, _, _>(|stat_data| stat_data.clone());
    context.bind(&stat_data_binding);

    rsx! {
        <>
        {VecTracker::from(stat_data_binding.get().party.iter().map(|data| {
            constructor! {
                <StatsBoard troop_name={data.troop_name.clone()} health_percent={data.health_percent} speed={data.speed} defence={data.defence}></StatsBoard>
            }
        }))}
        </>
    }
}


fn destroy_ui(mut cmd: Commands) {
    cmd.remove_resource::<BevyContext>();
}

fn update_stat_data(query: Query<(&Name, &Tag, &Stats)>, binding: Res<Binding<StatData>>) {
     
    let mut new_stat_data = StatData::default();

    for (name, tag, stats) in query.iter() {

        let health_percent = stats.health() as f32 / stats.base_health() as f32;
        let stat_entry = StatDataEntry { troop_name: name.into(),  health_percent, speed: stats.speed(), defence: stats.defence() };

        match *tag {
            Tag::Player => { new_stat_data.party.push(stat_entry); },
            Tag::Enemy => { new_stat_data.enemies.push(stat_entry); }
        }
    }

    binding.set(new_stat_data);
}
