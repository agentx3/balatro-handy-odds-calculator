use std::collections::{HashMap, HashSet};

use crate::{
    card::Suit::{self, Clubs, Diamonds, Hearts, Spades, Wild, NONE},
    card::{Card, Rank},
    deck::Deck,
};
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum PokerHand {
    Pair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
    RoyalFlush,
    FlushHouse,
    FiveOfAKind,
    FlushFive,
}

#[derive(Debug, Clone)]
pub struct Hand {
    pub cards: Vec<Card>,
}

impl From<Vec<Card>> for Hand {
    fn from(cards: Vec<Card>) -> Self {
        Hand { cards }
    }
}

fn create_hand_with_subbed_wilds(cards: &Vec<Card>) -> Hand {
    let mut new_cards = Vec::new();
    for card in cards.iter() {
        if card.suit == Wild {
            for suit in [Clubs, Diamonds, Hearts, Spades] {
                new_cards.push(Card {
                    rank: card.rank,
                    suit,
                });
            }
        } else {
            new_cards.push(card.clone());
        }
    }
    Hand { cards: new_cards }
}

impl Hand {
    // Check all hands in one pass (or minimal passes) for performance
    pub fn evaluate_poker_hands(&mut self) -> HashMap<PokerHand, u32> {
        let mut rank_map = HashMap::new();
        let mut hand_map = HashMap::new();

        // Variables to help full house and flush house
        let mut full_house_potential_threes: Vec<Vec<Card>> = Vec::new();
        let mut full_house_potential_pairs: Vec<Vec<Card>> = Vec::new();
        let mut flush_house_potential_threes: Vec<Vec<Card>> = Vec::new();
        let mut flush_house_potential_pairs: Vec<Vec<Card>> = Vec::new();

        // Variable to help with straights
        let mut curr_straight_streak = 1;
        let mut last_straight_rank: Rank = Rank::NONE;
        let mut potential_straight_suits: HashSet<Suit> = HashSet::new();
        let mut ace_suits_for_straight_flush: HashSet<Suit> = HashSet::new();
        let mut primed_for_low_ace_straight = false;

        let mut last_suit: Suit = NONE;
        let mut suit_streak = 0;

        // Used to distinguish flush from five-flush, only used in first loop
        let mut last_rank: Rank = Rank::NONE;
        let mut rank_streak = 0;

        // TODO: Make this more efficient, maybe we can avoid this loop?
        // Just need to get something working for now
        // Sort the wild cards
        let mut hand_with_subbed_wilds = create_hand_with_subbed_wilds(&self.cards);
        hand_with_subbed_wilds
            .cards
            .sort_by(|a, b| a.suit.cmp(&b.suit).then(a.rank.cmp(&b.rank)));

        for card in hand_with_subbed_wilds.cards.iter() {
            if card.suit == last_suit || (card.suit == Wild && last_suit != NONE) {
                suit_streak += 1;
            } else {
                // We moved to the next suit
                // Reset the streak and de-prime the low ace straight
                primed_for_low_ace_straight = false;
                suit_streak = 1;
                // Reset the straight streak too, since we check for
                // regular straights after this loop
                curr_straight_streak = 1;

                // Reset rank streak since we mainly care about suits here
                if card.rank == last_rank {
                    rank_streak += 1;
                } else {
                    rank_streak = 1;
                }
            }

            if last_straight_rank == card.rank {
                // Do nothing
            } else if card.rank == last_straight_rank.next() {
                curr_straight_streak += 1;
            } else {
                curr_straight_streak = 1;
            }

            // We have a 2-5 straight right now
            if card.rank == Rank::Five && curr_straight_streak == 4 {
                primed_for_low_ace_straight = true;
                ace_suits_for_straight_flush.insert(card.suit);
            }
            if suit_streak >= 5 && suit_streak > curr_straight_streak && rank_streak < 5 {
                hand_map.insert(PokerHand::Flush, 1);
            }
            if suit_streak >= 5 && rank_streak >= 5 {
                hand_map.insert(PokerHand::FlushFive, 1);
            }
            if curr_straight_streak >= 5 && suit_streak >= 5 {
                hand_map.insert(PokerHand::StraightFlush, 1);
            } else if primed_for_low_ace_straight
                && card.rank == Rank::Ace
                && ace_suits_for_straight_flush.contains(&card.suit)
            {
                hand_map.insert(PokerHand::StraightFlush, 1);
            }
            last_straight_rank = card.rank;
            last_suit = card.suit;
            last_rank = card.rank;
        }
        // Reset for the next loop
        last_suit = NONE;
        last_straight_rank = Rank::NONE;
        primed_for_low_ace_straight = false;
        ace_suits_for_straight_flush.clear();

        // Sort the cards by rank
        self.cards
            .sort_by(|a, b| a.rank.cmp(&b.rank).then(a.suit.cmp(&b.suit)));

        // Used to help finding flush house and full house
        let mut last_non_wild_suit: Suit = NONE;
        let mut flush_threes: HashMap<Suit, boolean> = HashMap::new();
        let mut flush_pairs: HashMap<Suit, boolean> = HashMap::new();
        let mut have_non_flush_threes = false;
        let mut have_non_flush_pairs = false;

        for (i, card) in self.cards.iter().enumerate() {
            let rank_count = rank_map.entry(card.rank).or_insert(0);
            *rank_count += 1;

            if card.suit == last_suit || (card.suit == Wild && last_suit != NONE) {
                suit_streak += 1;
            } else {
                if card.suit != Wild {
                    last_non_wild_suit = card.suit;
                }
                suit_streak = 1;
            }

            // Check for the hands that use duplicates
            match rank_count {
                2 => {
                    match hand_map.get(&PokerHand::Pair) {
                        Some(exists) => {
                            if *exists > 0 {
                                hand_map.insert(PokerHand::TwoPair, 1);
                            }
                            hand_map.insert(PokerHand::Pair, 1);
                        }
                        None => {
                            hand_map.insert(PokerHand::Pair, 1);
                        }
                    }

                    // Does this account for wilds?
                    if suit_streak >= 2 {
                        flush_house_potential_pairs.push(self.cards[i - 1..=i].to_vec());
                    } else {
                        full_house_potential_pairs.push(self.cards[i - 1..=i].to_vec());
                    }
                }
                3 => {
                    hand_map.insert(PokerHand::ThreeOfAKind, 1);
                    // If we have a triple here, that means we previously had a pair
                    // TODO: This doesn't account for wilds, move to the upper loop
                    if suit_streak >= 3 {
                        flush_house_potential_threes.push(self.cards[i - 2..=i].to_vec());
                        flush_house_potential_pairs.pop();
                    } else {
                        full_house_potential_threes.push(self.cards[i - 2..=i].to_vec());
                        full_house_potential_pairs.pop();
                    }
                }
                4 => {
                    hand_map.insert(PokerHand::FourOfAKind, 1);
                }
                5 => {
                    if suit_streak == 5 {
                        hand_map.insert(PokerHand::FlushFive, 1);
                    } else {
                        hand_map.insert(PokerHand::FiveOfAKind, 1);
                    }
                }
                _ => {
                    // In addition to a flush five, we also have a five-of-a-kind
                    if *rank_count > 5 && suit_streak < 5 {
                        hand_map.insert(PokerHand::FiveOfAKind, 1);
                    }
                }
            };
            // Don't look for more straights if we already have one
            if hand_map.contains_key(&PokerHand::Straight) {
                last_suit = card.suit;
                continue;
            }

            // We have an ace, we need to check for a low ace straight
            if card.rank == Rank::Ace && primed_for_low_ace_straight {
                // Need to make sure our ace doesn't force a straight flush
                let mut force_straight_flush = false;
                for suit in ace_suits_for_straight_flush.iter() {
                    if card.suit == *suit {
                        force_straight_flush = true;
                    }
                }
                if !force_straight_flush {
                    hand_map.insert(PokerHand::Straight, 1);
                }
            }

            // Track straights
            if card.rank == last_straight_rank.next() {
                curr_straight_streak += 1;
                last_straight_rank = card.rank;
                potential_straight_suits.insert(card.suit);
            } else if card.rank == last_straight_rank {
                // suit alternative
                potential_straight_suits.insert(card.suit);
            } else {
                // The card is not the next in the straight reset
                curr_straight_streak = 1;
                last_straight_rank = card.rank;
                potential_straight_suits.clear();
            }

            if curr_straight_streak == 4 && card.rank == Rank::Five {
                // We have a 2-5 straight
                primed_for_low_ace_straight = true;

                // We must not match the suit with our ace
                match potential_straight_suits.len() {
                    // Just cycles once
                    1 => {
                        for suit in potential_straight_suits.iter() {
                            ace_suits_for_straight_flush.insert(*suit);
                            ace_suits_for_straight_flush.insert(Wild);
                        }
                    }
                    // Need to check if the second suit is wild
                    2 => {
                        let mut non_wild_suit: Suit = NONE;
                        let mut has_wild: bool = false;
                        for suit in potential_straight_suits.iter() {
                            if *suit != Wild {
                                non_wild_suit = *suit;
                            } else {
                                has_wild = true;
                            }
                        }
                        if has_wild {
                            ace_suits_for_straight_flush.insert(non_wild_suit);
                            ace_suits_for_straight_flush.insert(Wild);
                        }
                    }
                    _ => {}
                }
            }
            if curr_straight_streak == 5 {
                // Already handled straight flushes
                if potential_straight_suits.len() > 1 {
                    hand_map.insert(PokerHand::Straight, 1);
                }
            }

            last_suit = card.suit;
        }
        // Check for flush house and full house in flush-house potentials
        // This part is probably the least efficient part of the code
        // Because each triplet or pair might consist of a standard suit and wilds
        // And we don't know whether the hand is bound to a suit or fully wild
        'outer: for pts in flush_house_potential_threes.iter() {
            let mut full_wild_triplet = true;
            let mut suit_triplet: Suit = NONE;
            for card in pts.iter() {
                if card.suit != Wild {
                    suit_triplet = card.suit;
                }
            }
            // If we have a full wild triplet, we have a flush house
            // Because it doesn't matter what the other two cards are
            if full_wild_triplet {
                hand_map.insert(PokerHand::FlushHouse, 1);

                // If we have full house we dont' have to continue checking
                if hand_map.contains_key(&PokerHand::FullHouse) {
                    break 'outer;
                }
            }

            for pps in flush_house_potential_pairs.iter() {
                let mut full_wild_pair = true;
                let mut suit_pair: Suit = NONE;
                for card in pps.iter() {
                    if card.suit == Wild {
                        // Do nothing
                    } else if card.suit == suit_triplet {
                        // Matching suit
                        hand_map.insert(PokerHand::FlushHouse, 1);
                    } else {
                        // We at least have a regular full house
                        hand_map.insert(PokerHand::FullHouse, 1);
                    }
                }
            }
        }
        println!(
            "potentials threes: {:#?}, \npotentials pairs: {:#?}",
            flush_house_potential_threes, flush_house_potential_pairs
        );

        // Check for full house in all 4 sets of potentials X_houses
        // If we have 2 different 3 pairs, or 1 3 pair and 1 2 pair
        // of different suits, we have a full house
        if full_house_potential_threes.len() > 1
            || (full_house_potential_threes.len() > 0 && full_house_potential_pairs.len() > 0)
        {
            hand_map.insert(PokerHand::FullHouse, 1);
        }
        println!("Full house potentials: {:#?}", full_house_potential_threes);
        println!("Full house potentials: {:#?}", full_house_potential_pairs);
        hand_map
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common_macros::hash_map;

    fn assert_expected_results(
        results: &HashMap<PokerHand, u32>,
        expected: &HashMap<PokerHand, u32>,
    ) {
        for (key, expected_value) in expected.iter() {
            println!("Checking {:?}... expected: {}", key, expected_value);
            match results.get(key) {
                Some(v) => {
                    assert_eq!(
                        expected[key], *v,
                        "Expected {:?} to be {}, got {}",
                        key, expected[key], *v
                    );
                }
                None => {
                    println!("Got results: {:?}", results);
                    assert_eq!(
                        *expected_value, 0,
                        "Expected {:?} to be {}, got None",
                        key, expected[key]
                    );
                }
            }
        }
    }

    fn test_hand_correctness(cards: Vec<Card>, expected: &HashMap<PokerHand, u32>) {
        let mut hand = Hand::from(cards);
        let results = hand.evaluate_poker_hands();
        assert_expected_results(&results, &expected);
    }
    #[test]
    fn it_evaluates_hand_correctly_1() {
        let cards: Vec<Card>;

        #[rustfmt::skip]
        {
            cards = vec![
                Card { rank: Rank::Two  , suit: Clubs    },
                Card { rank: Rank::Five , suit: Clubs    },
                Card { rank: Rank::Three, suit: Clubs    },
                Card { rank: Rank::Four , suit: Wild     },
                Card { rank: Rank::Eight, suit: Diamonds },
                Card { rank: Rank::Eight, suit: Diamonds },
                Card { rank: Rank::Eight, suit: Clubs    },
                Card { rank: Rank::Ace  , suit: Wild     },
                Card { rank: Rank::Ace  , suit: Diamonds },
            ];
        }

        let expected: HashMap<PokerHand, u32> = hash_map! {
            PokerHand::Pair => 1,
            PokerHand::TwoPair => 1,
            PokerHand::ThreeOfAKind => 1,
            PokerHand::Flush => 1,
            PokerHand::Straight => 1,
            PokerHand::FullHouse => 1,
            PokerHand::StraightFlush => 1,
        };
        test_hand_correctness(cards, &expected);
    }

    #[test]
    fn it_evaluates_hand_correctly_2() {
        let cards: Vec<Card>;

        #[rustfmt::skip]
        {
            cards = vec![
                Card { rank: Rank::Two  , suit: Clubs    },
                Card { rank: Rank::Five , suit: Clubs    },
                Card { rank: Rank::Three, suit: Clubs    },
                Card { rank: Rank::Four , suit: Hearts   },
                Card { rank: Rank::Eight, suit: Diamonds },
                Card { rank: Rank::Eight, suit: Diamonds },
                Card { rank: Rank::Eight, suit: Clubs    },
                Card { rank: Rank::Ace  , suit: Wild     },
                Card { rank: Rank::Ace  , suit: Diamonds },
            ];
        }

        let expected: HashMap<PokerHand, u32> = hash_map! {
            PokerHand::Pair => 1,
            PokerHand::TwoPair => 1,
            PokerHand::ThreeOfAKind => 1,
            PokerHand::Flush => 1,
            PokerHand::Straight => 1,
            PokerHand::FullHouse => 1,
        };
        test_hand_correctness(cards, &expected);
    }
    #[test]
    fn it_evaluates_hand_correctly_3() {
        let cards: Vec<Card>;

        #[rustfmt::skip]
        {
            cards = vec![
                Card { rank: Rank::Two  , suit: Clubs    },
                Card { rank: Rank::Five , suit: Clubs    },
                Card { rank: Rank::Three, suit: Clubs    },
                Card { rank: Rank::Four , suit: Clubs    },
                Card { rank: Rank::Four , suit: Hearts   },
                Card { rank: Rank::Ace  , suit: Diamonds },
            ];
        }

        let expected: HashMap<PokerHand, u32> = hash_map! {
            PokerHand::Pair => 1,
            PokerHand::Straight => 1,
        };
        test_hand_correctness(cards, &expected);
    }
    #[test]
    fn it_evaluates_hand_correctly_4() {
        let cards: Vec<Card>;

        #[rustfmt::skip]
        {
            cards = vec![
                Card { rank: Rank::Two  , suit: Clubs    },
                Card { rank: Rank::Five , suit: Diamonds },
                Card { rank: Rank::Three, suit: Diamonds },
                Card { rank: Rank::Four , suit: Diamonds },
                Card { rank: Rank::Ace  , suit: Diamonds },
            ];
        }

        let expected: HashMap<PokerHand, u32> = hash_map! {
            PokerHand::Straight => 1,
        };
        test_hand_correctness(cards, &expected);
    }
    #[test]
    fn it_evaluates_5_wild_flush_five() {
        let cards: Vec<Card>;

        #[rustfmt::skip]
        {
            cards = vec![
                Card { rank: Rank::Two  , suit: Wild    },
                Card { rank: Rank::Two  , suit: Wild    },
                Card { rank: Rank::Two  , suit: Wild    },
                Card { rank: Rank::Two  , suit: Wild    },
                Card { rank: Rank::Two  , suit: Wild    },
            ];
        }

        let expected: HashMap<PokerHand, u32> = hash_map! {
            PokerHand::FlushFive => 1,
        };
        test_hand_correctness(cards, &expected);
    }
    #[test]
    fn it_evalutes_flush_house() {
        let cards: Vec<Card>;

        #[rustfmt::skip]
        {
            cards = vec![
                Card { rank: Rank::Two  , suit: Wild    },
                Card { rank: Rank::Two  , suit: Wild    },
                Card { rank: Rank::Three  , suit: Clubs    },
                Card { rank: Rank::Three  , suit: Wild    },
                Card { rank: Rank::Three  , suit: Wild    },
                Card { rank: Rank::Ace  , suit: Clubs    },
            ];
        }

        let expected: HashMap<PokerHand, u32> = hash_map! {
            PokerHand::FlushHouse => 1,
        };
        test_hand_correctness(cards, &expected);
    }
}
