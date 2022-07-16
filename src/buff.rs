use bevy::prelude::*;

use crate::game::StartRoundEvent;

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
pub struct Stats {
    health: u32,
    speed: u32,
    defence: u32,

    buffs: Vec<Buff>,
}

impl Stats {
    pub fn new(health: u32, speed: u32, defence: u32) -> Self {
        Stats {
            health,
            speed,
            defence,

            buffs: vec!(),
        }
    }

    pub fn health(&self) -> u32 {
        self.health
    }
    pub fn take_damage(&mut self, amount: u32) {
        // take damage taking defense into account
        self.health = if amount > self.health {
            0
        } else {
            self.health - amount
        };
    }
    pub fn heal(&mut self, amount: u32) {
        // TODO max health
        self.health += amount;
    }
    pub fn is_dead(&self) -> bool {
        self.health == 0
    }

    pub fn speed(&self) -> u32 {
        self.speed
    }

    pub fn defence(&self) -> u32 {
        self.defence
    }

    pub fn clear_buffs(&mut self) {
        self.buffs.clear();
    }
    pub fn add_buff(&mut self, buff: Buff) {
        // remove any existing buffs of same type (discriminant on only enum variant and not
        // contents)
        self.buffs.retain(|b| std::mem::discriminant(&buff.buff_type) != std::mem::discriminant(&b.buff_type));
        self.buffs.push(buff);
    }
    pub fn get_buff(&self, buff: Buff) -> Option<Buff> {
        let find = self.buffs.iter().find(|b| std::mem::discriminant(&buff.buff_type) == std::mem::discriminant(&b.buff_type));
        find.map(|b| b.clone())
    }
    pub fn has_buff(&self, buff: Buff) -> bool {
        self.get_buff(buff).is_some()
    }
}

pub struct BuffPlugin;

impl Plugin for BuffPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(buff_lifetime_system);
    }
}

fn buff_lifetime_system(mut query: Query<&mut Stats>) {

    for mut effects in query.iter_mut() {
        
        // remove expired effects
        effects.buffs.retain(|buff| {

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
        for mut buff in effects.buffs.iter_mut() {

            if let BuffLifetime::Rounds(round) = buff.lifetime {
                buff.lifetime = BuffLifetime::Rounds(round-1);
            }

        }

    }
}

fn apply_start_round_buffs(mut events: EventReader<StartRoundEvent>, mut query: Query<(Entity, &mut Stats)>) {

    for _ in events.iter() {

        for (entity, mut stat) in query.iter_mut() {
            
            for buff in stat.buffs.iter() {
                
                /*
                match buff.buff_type {
                    BuffType::Regeneration { amount } => { stat.heal(amount as u32) },
                    BuffType::Burning { amount } => {},
                    BuffType::Poison { amount } => {},
                    _ => {}
                }
                */

            }

        }
    }

}
