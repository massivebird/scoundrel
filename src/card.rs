use strum::EnumIter;

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Suit {
    Hearts,
    Spades,
    Clubs,
    Diamonds,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Rank {
    Ace,
    Queen,
    King,
    Jack,

    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
}

#[derive(Debug)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}
