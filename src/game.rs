use crate::card::Suit;
use crate::deck::Deck;
use crate::player::Player;
use crate::room::Room;
use rand::rngs::ThreadRng;

pub struct Game {
    player: Player,
    deck: Deck,
    room: Room,
}

impl Game {
    pub fn new(rng: &mut ThreadRng) -> Self {
        let mut deck = Deck::generate(rng);
        let room = Room::from_deck(&mut deck);

        Self {
            player: Player::new(),
            deck,
            room,
        }
    }

    pub fn interact(&mut self, index: usize) {
        let Some(card) = self.room.pop_nth(index) else {
            return;
        };

        match card.suit {
            Suit::Hearts => self.player.heal(card.rank.value()),
            Suit::Diamonds => self.player.equip(card),
            Suit::Clubs | Suit::Spades => self.player.battle(card),
        }
    }

    pub fn print_game(&self) {
        for card in self.room.iter() {
            match card {
                Some(card) => print!("{card} "),
                None => print!("_ "),
            }
        }
        println!("\r");

        println!("{} HP, WPN: {:?}\r", self.player.hp(), self.player.weapon());
    }
}
