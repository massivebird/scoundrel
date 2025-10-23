use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use strum::IntoEnumIterator;

use crate::card::{Card, Rank, Suit};
use std::collections::VecDeque;

pub struct Deck(VecDeque<Card>);

impl Deck {
    /// Generates a shuffled Scoundrel deck.
    pub fn generate(rng: &mut ThreadRng) -> Self {
        let mut deck: VecDeque<Card> = VecDeque::new();

        for rank in Rank::iter() {
            for suit in Suit::iter() {
                // Omit face cards of diamonds and hearts.
                if matches!(rank, Rank::Queen | Rank::King | Rank::Jack | Rank::Ace)
                    && matches!(suit, Suit::Diamonds | Suit::Hearts)
                {
                    continue;
                }

                deck.push_front(Card { rank, suit });
            }
        }

        deck.make_contiguous().shuffle(rng);

        Self(deck)
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.0.pop_front()
    }

    /// Places a card at the bottom of the deck.
    ///
    /// I gotta rename this, man
    pub fn tuck(&mut self, card: Card) {
        self.0.push_back(card);
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
