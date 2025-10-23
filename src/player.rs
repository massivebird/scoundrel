use crate::card::Card;

pub struct Player {
    weapon: Option<Card>,
    slain_enemy: Option<Card>,
    hp: u32,
}

impl Player {
    pub const fn new() -> Self {
        Self {
            hp: 20,
            weapon: None,
            slain_enemy: None,
        }
    }

    pub const fn equip(&mut self, weapon: Card) {
        self.weapon = Some(weapon);
    }

    pub fn heal(&mut self, amount: u32) {
        self.hp = 20_u32.min(self.hp + amount);
    }

    pub fn battle(&mut self, enemy: Card) {
        if self.effective_power() >= enemy.rank.value() {
            self.slain_enemy = Some(enemy);
            return;
        }

        let enemy_power = enemy.rank.value();

        // Fresh weapon is being used against a higher power enemy.
        if self.weapon.is_some() && self.slain_enemy.is_none() {
            self.hp = self.hp.saturating_sub(enemy_power - self.effective_power());
            self.slain_enemy = Some(enemy);
            return;
        }

        self.hp = self.hp.saturating_sub(enemy_power);
    }

    fn effective_power(&self) -> u32 {
        self.slain_enemy
            .map(|card| card.rank.value())
            .or_else(|| self.weapon.map(|card| card.rank.value()))
            .unwrap_or_default()
    }

    pub const fn hp(&self) -> u32 {
        self.hp
    }

    pub const fn weapon(&self) -> Option<Card> {
        self.weapon
    }
}
