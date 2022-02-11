#[derive(Debug)]
pub enum Suit {
    CLUB,
    DIAMOND,
    HEART,
    SPADE,
}

impl Suit {
    pub fn from_int(i: u8) -> Self {
        match i {
            0 => Suit::CLUB,
            1 => Suit::DIAMOND,
            2 => Suit::HEART,
            3 => Suit::SPADE,
            _ => Suit::CLUB,
        }
    }
}
