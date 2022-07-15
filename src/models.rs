pub enum Side {
    Blank,
    Number(u32),
    Ability(String),
}

pub struct Dice {
    pub sides: [Side; 6],
}

impl Dice {
    /*
    pub fn new(ability_name: &str) -> Self {
        Dice {
            sides: [
                Side::Blank,
                Side::Number(2),
                Side::Number(3),
                Side::Number(4),
                Side::Number(5),
                Side::Ability(ability_name.into()),
            ]
        }
    }
    */

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

pub struct TroopPrefab {
    pub display_name: String,
    pub base_health: u32,
    pub sprite_index: usize,
}
