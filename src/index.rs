#[derive(Debug)]
pub enum Index {
    ACE = 1,
    TWO = 2,
    THREE = 3,
    FOUR = 4,
    FIVE = 5,
    SIX = 6,
    SEVEN = 7,
    EIGHT = 8,
    NINE = 9,
    TEN = 10,
    JACK = 11,
    QUEEN = 12,
    KING = 13,
}

impl Index {
    pub fn from_int(i: u8) -> Self {
        match i {
            0 => Index::ACE,
            1 => Index::TWO,
            2 => Index::THREE,
            3 => Index::FOUR,
            4 => Index::FIVE,
            5 => Index::SIX,
            6 => Index::SEVEN,
            7 => Index::EIGHT,
            8 => Index::NINE,
            9 => Index::TEN,
            10 => Index::JACK,
            11 => Index::QUEEN,
            12 => Index::KING,
            _ => Index::TWO,
        }
    }
}
