use bevy::prelude::*;

pub enum BuffType {

    // buffs
    Regeneration { amount: usize },
    Overheal { augment: Augment },
    HolyProtection,
    Invulnerable,
    Armored { augment: Augment },
    Haste { augment: Augment },
    DoubleStrike,

    // debuffs
    Burning { amount: usize },
    Poison { amount: usize },
    Stunned,
    Silenced,
    ArmorBroken,
    Confused,

}

pub enum BuffLifetime {
    Round,       // lasts one round
    Rounds(u32), // lasts specified number of rounds
    Level,       // lasts the entire level
}

pub enum Augment {
    Additive(f32),
    Multiplicative(f32),
}

pub struct Buff {
    pub buff_type: BuffType,
    pub lifetime: BuffLifetime,
}

impl Buff {
    pub fn regeneration() -> Self {
        Buff {
            buff_type: BuffType::Regeneration { amount: 10 },
            lifetime: BuffLifetime::Rounds(3)
        }
        
    }
    /*
    pub fn overheal() -> Self {

    }
    pub fn holy_protection() -> Self {

    }
    pub fn invulnerable() -> Self {

    }
    pub fn armored() -> Self {

    }
    pub fn haste() -> Self {

    }
    pub fn doubles_strike() -> Self {

    }
    pub fn burning() -> Self {

    }
    pub fn poison() -> Self {

    }
    pub fn stunned() -> Self {

    }
    pub fn silenced() -> Self {

    }
    pub fn armor_broken() -> Self {

    }
    pub fn confused() -> Self {

    }
    */
}

#[derive(Component)]
pub struct StatusEffects {
    pub effects: Vec<Buff>
}

pub struct BuffPlugin;

impl Plugin for BuffPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(buff_lifetime_system);
    }
}

pub fn buff_lifetime_system() {

}
