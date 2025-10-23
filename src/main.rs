use self::card::{Card, Rank, Suit};
use rand::seq::SliceRandom;
use std::collections::VecDeque;
use strum::IntoEnumIterator;

mod card;

fn main() {
    let mut deck: VecDeque<Card> = VecDeque::new();

    let mut rng = rand::rng();

    for rank in Rank::iter() {
        for suit in Suit::iter() {
            deck.push_front(Card { rank, suit });
        }
    }

    deck.make_contiguous().shuffle(&mut rng);

    dbg!(deck);
}
