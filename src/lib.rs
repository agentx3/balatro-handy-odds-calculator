#![feature(stmt_expr_attributes)]
use card::{Rank, Suit};
use hand::PokerHand;
use once_cell::sync::Lazy;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen;
use std::{collections::HashMap, sync::{Arc, Mutex}};
use wasm_bindgen::{ prelude::*}; // For initializing statics


// Enabling this should allow for parallelism in wasm
// But for some reason the web workers won't work
// so disabling for now
// #[cfg(target_arch = "wasm32")]
// pub use wasm_bindgen_rayon::init_thread_pool;

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
        Ok(deck) => {
            let mut _deck = DECK.lock().unwrap();
            *_deck = deck;
            drop(_deck);
            JsValue::from_f64(0f64)
        }
        Err(e) => {
            error(&e);
            JsValue::from_f64(1f64)
        }
    }
}

#[wasm_bindgen]
pub fn new_deck() -> JsValue {
    let deck = deck::Deck::new();
    deck.to_jsvalue()
}

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
    let deck_clone = Arc::new(deck.clone());
    drop(deck);
    // Use a parallel iterator to perform the trials in parallel
    let net_result: HashMap<PokerHand, u32> = (0..trials)
        .into_par_iter()
        .map(|_| {
            let mut hand = deck_clone.draw_hand(hand_size);
            hand.evaluate_poker_hands()
        })
        .reduce(
            || HashMap::new(),
            |mut acc, res| {
                // Combine results from each trial
                for (&k, &v) in res.iter() {
                    *acc.entry(k).or_insert(0) += v;
                }
                acc
            },
        );

    let net_result: HashMap<PokerHand, f64> = net_result
        .iter()
        .map(|(k, v)| (*k, *v as f64 / trials as f64))
        .collect();

    serde_wasm_bindgen::to_value(&net_result).unwrap()
    // let net_result = (0..trials)
    //     .into_par_iter()
    //     .map(|_| {
    //         let mut hand = deck_clone.draw_hand(hand_size);
    //         let result: HashMap<PokerHand, u32> = hand.evaluate_poker_hands();
    //         result
    //     })
    //     .reduce(
    //         || HashMap::new(),
    //         |mut acc, x| {
    //             for (&k, &v) in x.iter() {
    //                 let count = acc.entry(k).or_insert(0);
    //                 *count += v;
    //             }
    //             acc
    //         },
    //     );
    // let mut net_result = HashMap::new();
    // for _ in 0..trials {
    //     let mut hand = deck_clone.draw_hand(hand_size);
    //     let result: HashMap<PokerHand, u32> = hand.evaluate_poker_hands();
    //     for (&k, &v) in result.iter() {
    //         let count = net_result.entry(k).or_insert(0);
    //         *count += v;
    //     }
    // }
    // let net_result: HashMap<PokerHand, f64> = net_result
    //     .iter()
    //     .map(|(k, v)| (*k, *v as f64 / trials as f64))
    //     .collect();
    // serde_wasm_bindgen::to_value(&net_result).unwrap()
}
