use bevy::prelude::*;

pub struct Game {
    pub round: u32,
    pub troops: Vec<Entity>,
    pub enemies: Vec<Entity>,
}
