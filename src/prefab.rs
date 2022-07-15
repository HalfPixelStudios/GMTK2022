use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Stats {
    pub base_health: u32,
    pub base_speed: u32,
    pub base_defence: u32,
}

#[derive(Deserialize, Clone)]
pub enum Class {
    Warrior,
    Wizard,
    Archer,
    Cleric,
}

#[derive(Deserialize, Clone)]
pub struct TroopPrefab {
    pub display_name: String,
    pub class: Class,
    pub stats: Stats,
    pub default_dice: Dice,
    pub sprite_index: usize,
}

#[derive(Deserialize, Clone)]
pub struct EnemyPrefab {
    pub display_name: String,
    pub stats: Stats,
    pub default_dice: Dice,
    pub sprite_index: usize,
}

#[derive(Deserialize, Clone)]
pub enum Side {
    Blank,
    Number(u32),
    Ability(String),
}

#[derive(Deserialize, Clone)]
pub struct Dice {
    pub sides: [Side; 6],
}

impl Dice {
    pub fn roll(&self) -> &Side {
        use rand::{thread_rng, Rng};

        let face = thread_rng().gen_range(0..6);
        &self.sides[face]
    }

    pub fn replace(&mut self, index: usize, side: Side) {
        // TODO not safe
        self.sides[index] = side;
    }

    pub fn modify_number(&mut self, index: usize, modify: i32) {}
}
