use bevy::prelude::*;
use bevy_bobs::prefab::PrefabLib;
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

const RON_STRING: &str = r#"
{
    "warrior": (
        display_name: "warrior",
        stats: (
            base_health: 10,
            base_speed: 120,
            base_defence: 10,
        ),
        default_dice: (
            sides: (
                Blank,
                Number(2),
                Number(3),
                Number(4),
                Number(5),
                Ability("stun"),
            )
        ),
        sprite_index: 1,
    ),
    "orc": (
        display_name: "orc",
        stats: (
            base_health: 5,
            base_speed: 100,
            base_defence: 10,
        ),
        default_dice: (
            sides: (
                Number(2),
                Number(2),
                Number(2),
                Number(2),
                Number(2),
                Number(2),
            )
        ),
        sprite_index: 21,
    ),
}
"#;

pub struct PrefabPlugin;

impl Plugin for PrefabPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PrefabLib::<TroopPrefab>::new(RON_STRING));
    }
}
