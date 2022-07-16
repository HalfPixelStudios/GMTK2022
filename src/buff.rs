use bevy::prelude::*;

#[derive(Clone)]
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

#[derive(Clone)]
pub enum BuffLifetime {
    Round,       // lasts one round
    Rounds(u32), // lasts specified number of rounds
    Level,       // lasts the entire level
}

#[derive(Clone)]
pub enum Augment {
    Additive(f32),
    Multiplicative(f32),
}

#[derive(Clone)]
pub struct Buff {
    pub buff_type: BuffType,
    pub lifetime: BuffLifetime,
}

impl Buff {
    pub fn regeneration() -> Self {
        Buff {
            buff_type: BuffType::Regeneration { amount: 10 },
            lifetime: BuffLifetime::Rounds(3),
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

#[derive(Component, Deref, DerefMut)]
pub struct StatusEffects(pub Vec<Buff>);

impl StatusEffects {
    pub fn clear(&mut self) {
        self.0.clear();
    }
    pub fn add_effect(&mut self, buff: Buff) {
        // remove any existing buffs of same type (discriminant on only enum variant and not
        // contents)
        self.retain(|b| {
            std::mem::discriminant(&buff.buff_type) != std::mem::discriminant(&b.buff_type)
        });
        self.push(buff);
    }
}

pub struct BuffPlugin;

impl Plugin for BuffPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(buff_lifetime_system);
    }
}

pub fn buff_lifetime_system(mut query: Query<&mut StatusEffects>) {
    for mut effects in query.iter_mut() {
        // remove expired effects
        effects.retain(|buff| {
            if let BuffLifetime::Round = buff.lifetime {
                return false;
            }
            if let BuffLifetime::Rounds(rounds) = buff.lifetime {
                if rounds == 0 {
                    return false;
                }
            }
            true
        });

        // tick down rounds
        for mut buff in effects.iter_mut() {
            if let BuffLifetime::Rounds(round) = buff.lifetime {
                buff.lifetime = BuffLifetime::Rounds(round - 1);
            }
        }
    }
}
