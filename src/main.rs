#![feature(stmt_expr_attributes)]
mod card;
mod deck;
mod hand;
mod joker;
mod utils;

use std::collections::HashMap;
use std::env;

use card::{Card, Rank, Suit};
use deck::Deck;

use crate::hand::PokerHand;

fn main() {
    let mut deck = Deck::new();

    let mut results = HashMap::new();

    let trials = env::args()
        .nth(1)
        .unwrap_or("10000".to_string())
        .parse::<u64>()
        .unwrap_or(10000);
    for _ in 0..trials {
        let mut hand = deck.draw_hand(6);
        let result: HashMap<PokerHand, u32> = hand.evaluate_poker_hands();
        for (&k, &v) in result.iter() {
            let cnt = results.entry(k).or_insert(0) ;
            *cnt += v;
        }
    }
    for (k, v) in results.iter() {
        println!("Hand: {:?}, Freq, {}, Probability: {}", k, *v, (*v as f64) / trials as f64);
    }
    
}
