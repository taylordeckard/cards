use crate::card::Card;
use crate::suit::Suit;
use crate::index::Index;
use rand::thread_rng;
use rand::seq::SliceRandom;
use std::fmt;

const NUM_INDICES: u8 = 13;
const NUM_SUITS: u8 = 4;

pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    /** 
     * Constructor - Initialize the Deck and fill a vector with 52 cards
     */
    pub fn new() -> Self {
        let mut cards = Vec::new();
        for c in 0..NUM_INDICES {
            for s in 0..NUM_SUITS {
                cards.push(Card {
                    suit: Suit::from_int(s),
                    index: Index::from_int(c),
                });
            }
        }
        Self {
            cards,
        }
    }
    pub fn shuffle(&mut self) {
        let rng = &mut thread_rng();
        self.cards.shuffle(rng);
        println!("{}", self);
    }
}

/**
 * Implement trait to print all cards in the deck at once
 */
impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut out = String::from("");
        for c in &self.cards {
            out = match out.is_empty() {
                true => format!("{}", c),
                false => format!("{} {}", out, c),
            };
        }
        write!(f, "{}", out)
    }
}
