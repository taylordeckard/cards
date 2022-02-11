mod card;
mod suit;
mod deck;
mod index;
use crate::deck::Deck;

fn main() {
    let mut deck = Deck::new();
    deck.shuffle();
}
