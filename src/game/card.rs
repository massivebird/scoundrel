use colored::Colorize;
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
    pub const fn value(self) -> u32 {
        match self {
            Self::Ace => 14,
            Self::King => 13,
            Self::Queen => 12,
            Self::Jack => 11,
            Self::Ten => 10,
            Self::Nine => 9,
            Self::Eight => 8,
            Self::Seven => 7,
            Self::Six => 6,
            Self::Five => 5,
            Self::Four => 4,
            Self::Three => 3,
            Self::Two => 2,
        }
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let base = self.rank.to_string() + &self.suit.to_string();

        let color = match self.suit {
            Suit::Hearts => colored::Color::Red,
            Suit::Diamonds => colored::Color::Yellow,
            Suit::Spades | Suit::Clubs => colored::Color::Blue,
        };

        write!(f, "{}", base.color(color))
    }
}
