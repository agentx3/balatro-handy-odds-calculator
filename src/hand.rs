use std::collections::HashMap;

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
    // Check all hands in one pass for performance
    pub fn evaluate_poker_hands(&mut self) -> HashMap<PokerHand, u32> {
        let mut rank_map = HashMap::new();
        let mut suit_map = HashMap::new();

        let mut hand_map = HashMap::new();

        // Variables to help full house and flush house
        let mut full_house_potential_threes: Vec<Vec<Card>> = Vec::new();
        let mut full_house_potential_pairs: Vec<Vec<Card>> = Vec::new();
        let mut flush_house_potential_threes: Vec<Vec<Card>> = Vec::new();
        let mut flush_house_potential_pairs: Vec<Vec<Card>> = Vec::new();

        // Variable to help with straights
        let mut curr_straight_streak = 1;
        let mut last_straight_rank: Rank = Rank::NONE;

        let mut last_suit: Suit = NONE;
        let mut suit_streak = 0;

        // Sort the wild cards

        // TODO: Make this more efficient
        // Just need to get something working for now
        let mut hand_with_subbed_wilds = create_hand_with_subbed_wilds(&self.cards);
        hand_with_subbed_wilds
            .cards
            .sort_by(|a, b| a.suit.cmp(&b.suit).then(a.rank.cmp(&b.rank)));

        let mut primed_for_low_ace_straight = false;
        // Used for the loop after this one
        let mut has_ace = false;

        for card in hand_with_subbed_wilds.cards.iter() {
            if card.rank == Rank::Ace {
                has_ace = true;
            }
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
            }
            if suit_streak >= 5 && suit_streak > curr_straight_streak {
                hand_map.insert(PokerHand::Flush, 1);
            }
            if curr_straight_streak >= 5 && suit_streak >= 5 {
                hand_map.insert(PokerHand::StraightFlush, 1);
            } else if primed_for_low_ace_straight && card.rank == Rank::Ace {
                hand_map.insert(PokerHand::StraightFlush, 1);
            }
            last_straight_rank = card.rank;
            last_suit = card.suit;
        }
        // Reset for the next loop
        last_suit = NONE;
        last_straight_rank = Rank::NONE;
        // Sort the cards by rank
        self.cards
            .sort_by(|a, b| a.rank.cmp(&b.rank).then(a.suit.cmp(&b.suit)));

        for (i, card) in self.cards.iter().enumerate() {
            let rank_count = rank_map.entry(card.rank).or_insert(0);
            *rank_count += 1;
            let suit_count = suit_map.entry(card.suit).or_insert(0);
            *suit_count += 1;

            if card.suit == last_suit || (card.suit == Wild && last_suit != NONE) {
                suit_streak += 1;
            } else {
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
                    full_house_potential_pairs.push(self.cards[i - 1..=i].to_vec());
                    // TODO: This doesn't account for wilds, move to the upper loop
                    if suit_streak == 2 {
                        flush_house_potential_pairs.push(self.cards[i - 1..=i].to_vec());
                    }
                }
                3 => {
                    hand_map.insert(PokerHand::ThreeOfAKind, 1);
                    // If we have a triple here, that means we previously had a pair
                    full_house_potential_threes.push(self.cards[i - 2..=i].to_vec());
                    full_house_potential_pairs.pop();
                    // TODO: This doesn't account for wilds, move to the upper loop
                    if suit_streak == 3 {
                        flush_house_potential_threes.push(self.cards[i - 2..=i].to_vec());
                        flush_house_potential_pairs.pop();
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
            // Track straights
            if card.rank == last_straight_rank.next() {
                curr_straight_streak += 1;
                last_straight_rank = card.rank;
            } else if card.rank == last_straight_rank {
                // Do nothing
            } else {
                // The card is not the next in the straight
                curr_straight_streak = 1;
                last_straight_rank = card.rank;
            }
            if curr_straight_streak == 4 && has_ace && card.rank == Rank::Five {
                hand_map.insert(PokerHand::Straight, 1);
            }
            if curr_straight_streak == 5 {
                // Already handled straight flushes
                if suit_streak < 5 {
                    hand_map.insert(PokerHand::Straight, 1);
                }
            }
            last_suit = card.suit;
        }
        // Check for full house
        if full_house_potential_threes.len() > 1
            || (full_house_potential_threes.len() > 0 && full_house_potential_pairs.len() > 0)
        {
            hand_map.insert(PokerHand::FullHouse, 1);
        }
        // Check for flush house
        'outer: for i in 0..flush_house_potential_threes.len() {
            for j in 0..flush_house_potential_pairs.len() {
                if flush_house_potential_threes[i][0].rank == flush_house_potential_pairs[j][0].rank
                {
                    hand_map.insert(PokerHand::FlushHouse, 1);
                    break 'outer;
                }
            }
        }
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
}
