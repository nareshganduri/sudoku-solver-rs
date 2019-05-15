use std::convert::TryFrom;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Number {
    Empty,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
}

impl Number {
    pub fn inc(self) -> Self {
        match self {
            Number::Empty => Number::One,
            Number::One => Number::Two,
            Number::Two => Number::Three,
            Number::Three => Number::Four,
            Number::Four => Number::Five,
            Number::Five => Number::Six,
            Number::Six => Number::Seven,
            Number::Seven => Number::Eight,
            Number::Eight => Number::Nine,
            Number::Nine => unreachable!(),
        }
    }
}

impl From<Number> for u8 {
    fn from(x: Number) -> u8 {
        match x {
            Number::Empty => 0,
            Number::One => 1,
            Number::Two => 2,
            Number::Three => 3,
            Number::Four => 4,
            Number::Five => 5,
            Number::Six => 6,
            Number::Seven => 7,
            Number::Eight => 8,
            Number::Nine => 9,
        }
    }
}

pub struct TooLarge;

impl TryFrom<u8> for Number {
    type Error = TooLarge;

    fn try_from(x: u8) -> Result<Self, TooLarge> {
        match x {
            0 => Ok(Number::Empty),
            1 => Ok(Number::One),
            2 => Ok(Number::Two),
            3 => Ok(Number::Three),
            4 => Ok(Number::Four),
            5 => Ok(Number::Five),
            6 => Ok(Number::Six),
            7 => Ok(Number::Seven),
            8 => Ok(Number::Eight),
            9 => Ok(Number::Nine),
            _ => Err(TooLarge),
        }
    }
}