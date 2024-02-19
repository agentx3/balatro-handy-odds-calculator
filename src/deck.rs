use rand::seq::SliceRandom;
use wasm_bindgen::JsValue;

use crate::card::{Card, Rank, Suit};
use crate::hand::Hand;
use crate::utils::statistics::generate_random_numbers;
use js_sys::Object;

#[derive(Debug, Clone)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl From<Vec<Card>> for Deck {
    fn from(cards: Vec<Card>) -> Self {
        Deck { cards }
    }
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = Vec::new();
        for suit in &[Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades] {
            for rank in 2..=14 {
                cards.push(Card {
                    suit: *suit,
                    rank: Rank::from(rank), // Implement From trait for conversion
                });
            }
        }
        Deck { cards }
    }

    pub fn to_jsvalue(&self) -> JsValue {
        let obj = Object::new();
        let array = js_sys::Array::new();
        for card in &self.cards {
            array.push(&card.to_jsvalue());
        }
        unsafe {
            let _ = js_sys::Reflect::set(&obj, &JsValue::from_str("cards"), &JsValue::from(array));
        }

        JsValue::from(obj)
    }

    pub fn from_jsvalue(jsvalue: JsValue) -> Result<Self, String> {
        // Expect { cards: {{rank: int, suit: str}[]} }
        let obj = js_sys::Object::from(jsvalue);
        let cards = match js_sys::Reflect::get(&obj, &JsValue::from_str("cards")) {
            Ok(cards) => cards,
            Err(_) => return Err("Invalid deck object".to_string()),
        };
        if !js_sys::Array::is_array(&cards) {
            return Err("Invalid deck object".to_string());
        }
        let cards_arr = js_sys::Array::from(&cards);
        let mut cards_vec = Vec::new();

        for i in 0..cards_arr.length() {
            let suit = match js_sys::Reflect::get(&cards_arr.get(i), &JsValue::from_str("suit")) {
                Ok(suit) => match suit.as_string() {
                    Some(suit) => suit,
                    None => return Err("Invalid card object".to_string()),
                },
                Err(_) => return Err("Invalid card object".to_string()),
            };
            let rank = match js_sys::Reflect::get(&cards_arr.get(i), &JsValue::from_str("rank")) {
                Ok(rank) => match rank.as_f64() {
                    Some(rank) => rank,
                    None => return Err("Invalid card object".to_string()),
                },
                Err(_) => return Err("Invalid card object".to_string()),
            };
            cards_vec.push(Card {
                rank: Rank::from_int(rank as usize),
                suit: Suit::from_str(suit.as_str()),
            });
        }
        Ok(Deck::from(cards_vec))

    }

    pub fn reset(&mut self) {
        self.cards.clear();
        for suit in &[Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades] {
            for rank in 2..=14 {
                self.cards.push(Card {
                    suit: *suit,
                    rank: Rank::from(rank), // Implement From trait for conversion
                });
            }
        }
    }

    // Add a card to the deck, allowing duplicates
    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    // Remove a card from the deck (if it exists)
    pub fn remove_card(&mut self, card: &Card) {
        if let Some(pos) = self.cards.iter().position(|x| x == card) {
            self.cards.remove(pos);
        }
    }

    pub fn remove_suits(&mut self, suit: &Suit) {
        let mut i = 0;
        while i < self.cards.len() {
            if self.cards[i].suit == *suit {
                self.cards.remove(i);
            } else {
                i += 1;
            }
        }
    }
    pub fn remove_rank(&mut self, rank: &Rank, count: u64) {
        let mut i = 0;
        while i < self.cards.len() {
            if self.cards[i].rank == *rank {
                self.cards.remove(i);
            } else {
                i += 1;
                if i as u64 == count {
                    break;
                }
            }
        }
    }

    pub fn remove_ranks(&mut self, rank: &Rank) {
        let mut i = 0;
        while i < self.cards.len() {
            if self.cards[i].rank == *rank {
                self.cards.remove(i);
            } else {
                i += 1;
            }
        }
    }

    pub fn remove_suit(&mut self, suit: &Suit, count: u64) {
        let mut i = 0;
        while i < self.cards.len() {
            if self.cards[i].suit == *suit {
                self.cards.remove(i);
            } else {
                i += 1;
                if i as u64 == count {
                    break;
                }
            }
        }
    }

    pub fn draw_hand(&self, size: u8) -> Hand {
        let mut hand = Vec::new();
        let random_idx = generate_random_numbers(0, self.size() as u8 - 1u8, size);
        for i in random_idx {
            hand.push(self.cards[i]);
        }
        Hand::from(hand)
    }

    pub fn count_rank(&self, rank: &Rank) -> u64 {
        self.cards.iter().filter(|x| &x.rank == rank).count() as u64
    }

    pub fn count_suit(&self, suit: &Suit) -> u64 {
        self.cards.iter().filter(|x| &x.suit == suit).count() as u64
    }

    pub fn count_card(&self, card: &Card) -> u64 {
        self.cards.iter().filter(|x| *x == card).count() as u64
    }
    // Shuffle the deck
    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.cards.shuffle(&mut rng);
    }
    pub fn size(&self) -> usize {
        self.cards.len()
    }
    pub fn sort_by_rank(&mut self) {
        self.cards.sort_by(|a, b| a.rank.cmp(&b.rank));
    }
    pub fn sort_by_suit(&mut self) {
        self.cards.sort_by(|a, b| a.suit.cmp(&b.suit));
    }
}
