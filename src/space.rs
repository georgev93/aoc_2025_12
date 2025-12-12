use std::fmt::{Debug, Display, Formatter, Result};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum Space {
    Occupied,
    Pocket,
    Free,
}

impl Space {
    pub fn new(input: char) -> Self {
        match input {
            '#' => Self::Occupied,
            '.' => Self::Free,
            'o' => Self::Pocket,
            _ => panic!("Tried to parse a Space character unsuccessfully"),
        }
    }
}

mod formatting {
    use super::*;

    impl Display for Space {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            match self {
                Self::Occupied => write!(f, "#"),
                Self::Free => write!(f, "."),
                Self::Pocket => write!(f, "o"),
            }
        }
    }

    impl Debug for Space {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            Display::fmt(self, f)
        }
    }
}
