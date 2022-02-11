use crate::suit::Suit;
use crate::index::Index;
use std::fmt;

#[derive(Debug)]
pub struct Card {
    pub suit: Suit,
    pub index: Index,
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let index = match self.index {
            Index::ACE => "A",
            Index::TWO => "2",
            Index::THREE => "3",
            Index::FOUR => "4",
            Index::FIVE => "5",
            Index::SIX => "6",
            Index::SEVEN => "7",
            Index::EIGHT => "8",
            Index::NINE => "9",
            Index::TEN => "10",
            Index::JACK => "J",
            Index::QUEEN => "Q",
            Index::KING => "K",
        };
        let suit = match self.suit {
            Suit::CLUB => "♣",
            Suit::DIAMOND => "♦",
            Suit::HEART => "♥",
            Suit::SPADE => "♠",
        };
        write!(f, "{}{}", index, suit)
    }
}
