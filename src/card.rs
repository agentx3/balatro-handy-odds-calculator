use core::fmt;
use std::fmt::{Display, Formatter};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Suit {
    NONE = 0,
    Clubs,
    Diamonds,
    Hearts,
    Spades,
    Wild,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Rank {
    NONE = 0,
    Two = 2,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Rank {
    pub fn from_str(s: &str) -> Result<Rank, &'static str> {
        match s {
            "Two" => Ok(Rank::Two),
            "Three" => Ok(Rank::Three),
            "Four" => Ok(Rank::Four),
            "Five" => Ok(Rank::Five),
            "Six" => Ok(Rank::Six),
            "Seven" => Ok(Rank::Seven),
            "Eight" => Ok(Rank::Eight),
            "Nine" => Ok(Rank::Nine),
            "Ten" => Ok(Rank::Ten),
            "J" => Ok(Rank::Jack),
            "Q" => Ok(Rank::Queen),
            "K" => Ok(Rank::King),
            "A" => Ok(Rank::Ace),
            _ => Err("Invalid rank"),
        }
    }

    pub fn next(&self)->Rank{
        match self{
            Rank::Two=>Rank::Three,
            Rank::Three=>Rank::Four,
            Rank::Four=>Rank::Five,
            Rank::Five=>Rank::Six,
            Rank::Six=>Rank::Seven,
            Rank::Seven=>Rank::Eight,
            Rank::Eight=>Rank::Nine,
            Rank::Nine=>Rank::Ten,
            Rank::Ten=>Rank::Jack,
            Rank::Jack=>Rank::Queen,
            Rank::Queen=>Rank::King,
            Rank::King=>Rank::Ace,
            Rank::Ace=>Rank::Two,
            Rank::NONE=>Rank::NONE,
        }
    }

    pub fn prev(&self)->Rank{
        match self{
            Rank::Two=>Rank::Ace,
            Rank::Three=>Rank::Two,
            Rank::Four=>Rank::Three,
            Rank::Five=>Rank::Four,
            Rank::Six=>Rank::Five,
            Rank::Seven=>Rank::Six,
            Rank::Eight=>Rank::Seven,
            Rank::Nine=>Rank::Eight,
            Rank::Ten=>Rank::Nine,
            Rank::Jack=>Rank::Ten,
            Rank::Queen=>Rank::Jack,
            Rank::King=>Rank::Queen,
            Rank::Ace=>Rank::King,
            Rank::NONE=>Rank::NONE,
        }
    }
    // pub fn from_int(i: u8) -> Result<Rank, &'static str> {
    //     match i {
    //         2 => Ok(Rank::Two),
    //         3 => Ok(Rank::Three),
    //         4 => Ok(Rank::Four),
    //         5 => Ok(Rank::Five),
    //         6 => Ok(Rank::Six),
    //         7 => Ok(Rank::Seven),
    //         8 => Ok(Rank::Eight),
    //         9 => Ok(Rank::Nine),
    //         10 => Ok(Rank::Ten),
    //         11 => Ok(Rank::Jack),
    //         12 => Ok(Rank::Queen),
    //         13 => Ok(Rank::King),
    //         14 => Ok(Rank::Ace),
    //         _ => Err("Invalid rank"),
    //     }
    // }

    // pub fn to_int(&self) -> u8 {
    //     *self as u8
    // }

    // pub fn str_to_int(s: &str) -> Result<u8, &'static str> {
    //     match s {
    //         "Two" => Ok(2),
    //         "Three" => Ok(3),
    //         "Four" => Ok(4),
    //         "Five" => Ok(5),
    //         "Six" => Ok(6),
    //         "Seven" => Ok(7),
    //         "Eight" => Ok(8),
    //         "Nine" => Ok(9),
    //         "Ten" => Ok(10),
    //         "J" => Ok(11),
    //         "Q" => Ok(12),
    //         "K" => Ok(13),
    //         "A" => Ok(14),
    //         _ => Err("Invalid rank"), 
    //     }
    // }

    pub fn to_str(&self) -> &'static str {
        match self {
            Rank::NONE => "NONE",
            Rank::Two => "Two",
            Rank::Three => "Three",
            Rank::Four => "Four",
            Rank::Five => "Five",
            Rank::Six => "Six",
            Rank::Seven => "Seven",
            Rank::Eight => "Eight",
            Rank::Nine => "Nine",
            Rank::Ten => "Ten",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
            Rank::Ace => "A",
        }
    }

    // pub fn all_ranks() -> Vec<Rank> {
    //     vec![
    //         Rank::Two,
    //         Rank::Three,
    //         Rank::Four,
    //         Rank::Five,
    //         Rank::Six,
    //         Rank::Seven,
    //         Rank::Eight,
    //         Rank::Nine,
    //         Rank::Ten,
    //         Rank::Jack,
    //         Rank::Queen,
    //         Rank::King,
    //         Rank::Ace,
    //     ]
    // }
}

impl Display for Rank {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let txt = match self.to_str() {
            "J" => "Jack",
            "Q" => "Queen",
            "K" => "King",
            "A" => "Ace",
            _ => self.to_str(),
        };
        write!(f, "{}", txt)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Card {
    pub fn next(&self)->Card{
        Card{
            rank:self.rank.next(),
            suit:self.suit,
        }
    }
    pub fn prev(&self)->Card{
        Card{
            rank:self.rank.prev(),
            suit:self.suit,
        }
    }
}

impl From<usize> for Rank {
    fn from(item: usize) -> Self {
        match item {
            2 => Rank::Two,
            3 => Rank::Three,
            4 => Rank::Four,
            5 => Rank::Five,
            6 => Rank::Six,
            7 => Rank::Seven,
            8 => Rank::Eight,
            9 => Rank::Nine,
            10 => Rank::Ten,
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            14 => Rank::Ace,
            _ => panic!("Invalid rank"),
        }
    }
}
