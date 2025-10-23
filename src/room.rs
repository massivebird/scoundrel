use crate::card::Card;
use crate::deck::Deck;

pub struct Room([Option<Card>; 4]);

impl Room {
    // Creates a full room by drawing four cards from the deck.
    pub fn from_deck(deck: &mut Deck) -> Self {
        let mut room: [Option<Card>; 4] = [None; 4];

        for space in &mut room {
            *space = deck.draw();
        }

        Self(room)
    }

    pub fn pop_nth(&mut self, index: usize) -> Option<Card> {
        self.0.get_mut(index)?.take()
    }

    // Fills vacant spaces in the room by drawing from the deck, as long as the
    // deck still contains cards.
    pub fn try_fill(&mut self, deck: &mut Deck) {
        for space in self.0.iter_mut().filter(|op| op.is_none()) {
            *space = deck.draw();
        }
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Option<Card>> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, Option<Card>> {
        self.0.iter_mut()
    }
}
