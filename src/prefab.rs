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
    pub default_dice: DicePrefab,
    pub sprite_index: usize,
}

#[derive(Deserialize, Clone)]
pub enum Side {
    Blank,
    Number(u32),
    Ability(String),
}

#[derive(Deserialize, Clone)]
pub struct DicePrefab {
    pub sides: [Side; 6],
}
