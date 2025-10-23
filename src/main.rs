use self::{card::Card, deck::Deck};

mod card;
mod deck;

fn main() {
    let mut rng = rand::rng();

    let mut deck = Deck::generate(&mut rng);

    let mut room: [Option<Card>; 4] = [const { None }; 4];

    // Initial populate
    for slot in &mut room {
        *slot = deck.draw();
    }

    dbg!(room);
}
