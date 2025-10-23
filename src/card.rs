use strum::{Display, EnumIter};

#[derive(Copy, Clone, Debug, Display, EnumIter)]
pub enum Suit {
    #[strum(to_string = "")]
    Hearts,
    #[strum(to_string = "󰣑")]
    Spades,
    #[strum(to_string = "󰣎")]
    Clubs,
    #[strum(to_string = "󰣏")]
    Diamonds,
}

#[derive(Copy, Clone, Debug, Display, EnumIter)]
pub enum Rank {
    #[strum(to_string = "A")]
    Ace,
    #[strum(to_string = "Q")]
    Queen,
    #[strum(to_string = "K")]
    King,
    #[strum(to_string = "J")]
    Jack,

    #[strum(to_string = "2")]
    Two,
    #[strum(to_string = "3")]
    Three,
    #[strum(to_string = "4")]
    Four,
    #[strum(to_string = "5")]
    Five,
    #[strum(to_string = "6")]
    Six,
    #[strum(to_string = "7")]
    Seven,
    #[strum(to_string = "8")]
    Eight,
    #[strum(to_string = "9")]
    Nine,
    #[strum(to_string = "10")]
    Ten,
}

#[derive(Copy, Clone, Debug)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Rank {
    pub fn value(&self) -> u32 {
        match self {
            Rank::Ace => 14,
            Rank::King => 13,
            Rank::Queen => 12,
            Rank::Jack => 11,
            Rank::Ten => 10,
            Rank::Nine => 9,
            Rank::Eight => 8,
            Rank::Seven => 7,
            Rank::Six => 6,
            Rank::Five => 5,
            Rank::Four => 4,
            Rank::Three => 3,
            Rank::Two => 2,
        }
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.rank, self.suit)
    }
}
