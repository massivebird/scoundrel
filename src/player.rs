use crate::card::Card;

pub struct Player {
    weapon: Option<Card>,
    hp: u32,
}

impl Player {
    pub const fn new() -> Self {
        Self {
            weapon: None,
            hp: 20,
        }
    }
}
