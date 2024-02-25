use core::fmt;
use std::fmt::{Display, Formatter};

use js_sys::Object;
use wasm_bindgen::JsValue;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Suit {
    NONE = 0,
    Clubs,
    Diamonds,
    Hearts,
    Spades,
    Wild,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
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

impl Suit {
    pub fn to_str(&self) -> &'static str {
        match self {
            Suit::NONE => "NONE",
            Suit::Clubs => "Clubs",
            Suit::Diamonds => "Diamonds",
            Suit::Hearts => "Hearts",
            Suit::Spades => "Spades",
            Suit::Wild => "Wild",
        }
    }

    pub fn from_str(s: &str) -> Suit {
        match s {
            "Clubs" => Suit::Clubs,
            "Diamonds" => Suit::Diamonds,
            "Hearts" => Suit::Hearts,
            "Spades" => Suit::Spades,
            "Wild" => Suit::Wild,
            _ => Suit::NONE,
        }
    }

    pub fn from_jsvalue(jsvalue: JsValue) -> Result<Suit, &'static str> {
        let suit = match jsvalue.as_string() {
            Some(suit) => suit,
            None => return Err("Invalid suit object"),
        };
        Ok(Suit::from_str(&suit))
    }
}

impl Rank {
    pub fn from_jsvalue(jsvalue: JsValue) -> Result<Rank, &'static str> {
        let rank = match jsvalue.as_f64() {
            Some(rank) => rank as usize,
            None => return Err("Invalid rank object"),
        };
        Ok(Rank::from(rank))
    }

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

    pub fn next(&self) -> Rank {
        match self {
            Rank::Two => Rank::Three,
            Rank::Three => Rank::Four,
            Rank::Four => Rank::Five,
            Rank::Five => Rank::Six,
            Rank::Six => Rank::Seven,
            Rank::Seven => Rank::Eight,
            Rank::Eight => Rank::Nine,
            Rank::Nine => Rank::Ten,
            Rank::Ten => Rank::Jack,
            Rank::Jack => Rank::Queen,
            Rank::Queen => Rank::King,
            Rank::King => Rank::Ace,
            Rank::Ace => Rank::Two,
            Rank::NONE => Rank::NONE,
        }
    }

    pub fn prev(&self) -> Rank {
        match self {
            Rank::Two => Rank::Ace,
            Rank::Three => Rank::Two,
            Rank::Four => Rank::Three,
            Rank::Five => Rank::Four,
            Rank::Six => Rank::Five,
            Rank::Seven => Rank::Six,
            Rank::Eight => Rank::Seven,
            Rank::Nine => Rank::Eight,
            Rank::Ten => Rank::Nine,
            Rank::Jack => Rank::Ten,
            Rank::Queen => Rank::Jack,
            Rank::King => Rank::Queen,
            Rank::Ace => Rank::King,
            Rank::NONE => Rank::NONE,
        }
    }

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

    pub fn to_int(&self) -> usize {
        match self {
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 11,
            Rank::Queen => 12,
            Rank::King => 13,
            Rank::Ace => 14,
            Rank::NONE => 0,
        }
    }

    pub fn from_int(i: usize) -> Rank {
        match i {
            1 => Rank::Ace,
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
            _ => Rank::NONE,
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
    pub fn next(&self) -> Card {
        Card {
            rank: self.rank.next(),
            suit: self.suit,
        }
    }
    pub fn prev(&self) -> Card {
        Card {
            rank: self.rank.prev(),
            suit: self.suit,
        }
    }

    pub fn to_str(&self) -> String {
        format!("{} of {}", self.rank.to_str(), self.suit.to_str())
    }

    pub fn to_jsvalue(&self) -> JsValue {
        let obj = Object::new();
        let _ = js_sys::Reflect::set(
            &obj,
            &JsValue::from_str("rank"),
            &JsValue::from_f64(self.rank.to_int() as f64),
        );
        let _ = js_sys::Reflect::set(
            &obj,
            &JsValue::from_str("suit"),
            &JsValue::from_str(self.suit.to_str()),
        );
        JsValue::from(obj)
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
