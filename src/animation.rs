use bevy::prelude::*;
use std::collections::*;

use bevy_asset_ron::*;
use serde::*;
#[derive(Debug, Deserialize, Clone, Eq, PartialEq, Hash)]
pub enum AniState {
    Idle,
    Attack,
}
#[derive(Component)]
pub struct Animation {
    pub timer: Timer,
    pub state: AniState,
    pub data: HashMap<AniState, Vec2>,
    pub finished: bool,
    pub index: i32,
}
impl Animation {
    pub fn set_state(&mut self, state: AniState) {
        self.state = state;
        self.index = -1;
    }
}

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(animate);
    }
}

pub fn animate(time: Res<Time>, mut animations: Query<(&mut Animation, &mut TextureAtlasSprite)>) {
    for (mut ani, mut sprite) in animations.iter_mut() {
        let ani_range = ani.data.get(&ani.state).unwrap().clone();

        if ani.index == -1 {
            ani.index = ani_range.x as i32;
            return;
        }
        ani.timer.tick(time.delta());
        if ani.timer.just_finished() {
            if ani.index >= ani_range.y as i32 {
                ani.index = ani_range.x as i32;
                ani.finished = true;
            }
            ani.index += 1;
            sprite.index = ani.index as usize;
        }
    }
}
