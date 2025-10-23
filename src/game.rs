use crate::card::Suit;
use crate::deck::Deck;
use crate::player::Player;
use crate::room::Room;
use rand::rngs::ThreadRng;

pub struct Game {
    player: Player,
    deck: Deck,
    room: Room,
    /// Only one heal per room, the rest are discarded.
    has_healed: bool,
    /// Cannot avoid two rooms in a row.
    has_avoided: bool,
}

impl Game {
    pub fn new(rng: &mut ThreadRng) -> Self {
        let mut deck = Deck::generate(rng);
        let room = Room::from_deck(&mut deck);

        Self {
            player: Player::new(),
            deck,
            room,
            has_healed: false,
            has_avoided: false,
        }
    }

    pub fn interact(&mut self, index: usize) {
        let Some(card) = self.room.pop_nth(index) else {
            return;
        };

        match card.suit {
            Suit::Hearts => {
                if !self.has_healed {
                    self.player.heal(card.rank.value());
                    self.has_healed = true;
                }
            }
            Suit::Diamonds => self.player.equip(card),
            Suit::Clubs | Suit::Spades => self.player.battle(card),
        }

        if self.room.vacancies() == 3 {
            self.room.try_fill(&mut self.deck);
            self.has_healed = false;
        }
    }

    pub fn is_over(&self) -> bool {
        self.player.hp() == 0 || (self.room.vacancies() == 4 && self.deck.is_empty())
    }

    pub fn print_game(&self) {
        for card in self.room.iter() {
            match card {
                Some(card) => print!("{card} "),
                None => print!("__ "),
            }
        }
        println!("\r");

        println!(
            "{} HP, WPN: {}, POW: {}\r",
            self.player.hp(),
            self.player
                .weapon()
                .map_or("None".to_owned(), |c| c.to_string()),
            self.player.power()
        );
    }

    /// Avoiding fails if:
    /// 1. The player avoided the previous room, or
    /// 2. This room is not full (player has entered, or this is the final room).
    pub fn try_avoid(&mut self) {
        if self.has_avoided || self.room.vacancies() > 0 {
            return;
        }

        for card in self.room.iter_mut().filter(|c| c.is_some()) {
            self.deck.tuck(card.take().unwrap());
        }

        self.has_avoided = true;

        self.room.try_fill(&mut self.deck);
    }
}
