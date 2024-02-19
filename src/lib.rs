#![feature(stmt_expr_attributes)]
use card::{Rank, Suit};
use hand::PokerHand;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen;
use std::{collections::HashMap, sync::Mutex};
use wasm_bindgen::{convert::WasmAbi, prelude::*}; // For initializing statics
mod card;
mod deck;
mod hand;
mod utils;
use deck::Deck;

// static mut DECK: Deck = Deck { cards: Vec::new() };
static DECK: Lazy<Mutex<Deck>> = Lazy::new(|| Mutex::new(Deck::new()));

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn error(msg: &str);
}

#[wasm_bindgen]
pub fn parse_deck(deck: JsValue) -> JsValue {
    let deck = deck::Deck::from_jsvalue(deck);
    match deck {
        Ok(deck) => deck.to_jsvalue(),
        Err(e) => {
            error(&e);
            JsValue::NULL
        }
    }
}
#[wasm_bindgen]
pub fn new_deck() -> JsValue {
    let deck = deck::Deck::new();
    deck.to_jsvalue()
}

// pub fn add_card(suit: JsValue, rank: JsValue) -> Result<(), JsValue> {
//     let suit = card::Suit::from_jsvalue(suit);
//     let rank = card::Rank::from_jsvalue(rank);
//     match (suit, rank) {
//         (Ok(suit), Ok(rank)) => {
//             match DECK.lock() {
//                 Ok(mut d) => {
//                     d.add_card(card::Card { suit, rank });
//                 }
//                 Err(e) => {
//                     error(&format!("Failed to lock deck: {}", e));
//                     return Err(JsValue::from_str("Failed to lock deck"));
//                 }
//             }
//             Ok(())
//         }
//         _ => Err(JsValue::from_str("Invalid suit or rank")),
//     }
// }
#[wasm_bindgen]
pub fn add_card(suit: String, rank: i32) -> Result<(), JsValue> {
    match DECK.lock() {
        Ok(mut d) => {
            d.add_card(card::Card {
                suit: Suit::from_str(suit.as_str()),
                rank: Rank::from_int(rank as usize),
            });
        }
        Err(e) => {
            error(&format!("Failed to lock deck: {}", e));
            return Err(JsValue::from_str("Failed to lock deck"));
        }
    }
    Ok(())
}
#[wasm_bindgen]
pub fn remove_card(suit: JsValue, rank: JsValue) -> Result<(), JsValue> {
    let suit = card::Suit::from_jsvalue(suit);
    let rank = card::Rank::from_jsvalue(rank);
    match (suit, rank) {
        (Ok(suit), Ok(rank)) => {
            match DECK.lock() {
                Ok(mut d) => {
                    d.remove_card(&card::Card { suit, rank });
                }
                Err(e) => {
                    error(&format!("Failed to lock deck: {}", e));
                    return Err(JsValue::from_str("Failed to lock deck"));
                }
            }
            Ok(())
        }
        _ => Err(JsValue::from_str("Invalid suit or rank")),
    }
}

#[wasm_bindgen]
pub fn show_deck() -> JsValue {
    match DECK.lock() {
        Ok(d) => d.to_jsvalue(),
        Err(e) => {
            error(&format!("Failed to lock deck: {}", e));
            JsValue::NULL
        }
    }
}

#[allow(dead_code)]
#[wasm_bindgen]
#[derive(Serialize, Deserialize)]
pub struct PokerHandResult {
    hand: PokerHand,
    freq: u32,
    probability: f64,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
#[wasm_bindgen]
pub struct PokerHandResults {
    results: Vec<PokerHandResult>,
}


#[wasm_bindgen]
pub fn draw_trial(hand_size: u8, trials: u32) -> JsValue {
    let deck = match DECK.lock() {
        Ok(d) => d,
        Err(e) => {
            error(&format!("Failed to lock deck: {}", e));
            return JsValue::NULL;
        }
    };
    let deck_clone = deck.clone();
    drop(deck);
    let mut net_result = HashMap::new();
    for _ in 0..trials {
        let mut hand = deck_clone.draw_hand(hand_size);
        let result: HashMap<PokerHand, u32> = hand.evaluate_poker_hands();
        for (&k, &v) in result.iter() {
            let count = net_result.entry(k).or_insert(0);
            *count += v;
        }
    }
    serde_wasm_bindgen::to_value(&net_result).unwrap()
}
