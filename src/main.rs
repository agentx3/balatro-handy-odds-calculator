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
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;

use crate::hand::PokerHand;

fn main() {
    let mut deck = Deck::new();

    let mut results: HashMap<PokerHand, f64> = HashMap::new();

    let trials = env::args()
        .nth(1)
        .unwrap_or("10000".to_string())
        .parse::<u64>()
        .unwrap_or(10000);

    let net_result: HashMap<PokerHand, u32> = (0..trials)
        .into_par_iter()
        .map(|_| {
            let mut hand = deck.draw_hand(5);
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

    let results: HashMap<PokerHand, f64> = net_result
        .iter()
        .map(|(k, v)| (*k, *v as f64 / trials as f64))
        .collect();

    for (k, v) in results.iter() {
        println!(
            "Hand: {:?}, Probability: {}",
            k,
            *v,
        );
    }
}
