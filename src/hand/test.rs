use common_macros::hash_map;

#[cfg(test)]
use super::*;
use crate::{
    card::Card,
    hand::{Hand, PokerHand},
};
use std::collections::HashMap;

fn assert_expected_results(results: &HashMap<PokerHand, u32>, expected: &HashMap<PokerHand, u32>) {
    for (key, real_value) in results.iter() {
        println!(
            "Checking {:?}... expected: {}",
            key,
            expected.get(key).unwrap_or(&0)
        );
        match expected.get(key) {
            Some(v) => {
                assert_eq!(
                    results[key], *v,
                    "Expected {:?} to be {}, got {}",
                    key, v, real_value
                );
            }
            None => {
                panic!("Unexpected key: {:?}", key);
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
        PokerHand::FourOfAKind => 1,
        PokerHand::ThreeOfAKind => 1,
        PokerHand::Pair => 1,
    };
    test_hand_correctness(cards, &expected);
}

#[test]
fn it_evaluates_5_of_a_kind_with_wilds() {
    let cards: Vec<Card>;

    #[rustfmt::skip]
        {
            cards = vec![
                Card { rank: Rank::Two  , suit: Wild    },
                Card { rank: Rank::Two  , suit: Clubs   },
                Card { rank: Rank::Two  , suit: Hearts  },
                Card { rank: Rank::Two  , suit: Wild    },
                Card { rank: Rank::Two  , suit: Wild    },
            ];
        }

    let expected: HashMap<PokerHand, u32> = hash_map! {
        PokerHand::FiveOfAKind => 1,
        PokerHand::FourOfAKind => 1,
        PokerHand::ThreeOfAKind => 1,
        PokerHand::Pair => 1,
    };
    test_hand_correctness(cards, &expected);
}
#[test]
fn it_evaluates_5_of_a_kind_and_flush_five_with_wilds() {
    let cards: Vec<Card>;

    #[rustfmt::skip]
        {
            cards = vec![
                Card { rank: Rank::Two  , suit: Wild    },
                Card { rank: Rank::Two  , suit: Clubs   },
                Card { rank: Rank::Two  , suit: Hearts  },
                Card { rank: Rank::Two  , suit: Wild    },
                Card { rank: Rank::Two  , suit: Wild    },
                Card { rank: Rank::Two  , suit: Wild    },
            ];
        }

    let expected: HashMap<PokerHand, u32> = hash_map! {
        PokerHand::FlushFive => 1,
        PokerHand::FiveOfAKind => 1,
        PokerHand::FourOfAKind => 1,
        PokerHand::ThreeOfAKind => 1,
        PokerHand::Pair => 1,
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
        PokerHand::ThreeOfAKind => 1,
        PokerHand::TwoPair => 1,
        PokerHand::Pair => 1,
        PokerHand::FullHouse => 1,
        PokerHand::Flush => 1,
    };
    test_hand_correctness(cards, &expected);
}

#[test]
fn test_simple_flush() {
    let cards: Vec<Card>;
    #[rustfmt::skip]
        {
            cards = vec![
                Card { rank: Rank::Two, suit: Hearts },
                Card { rank: Rank::Three, suit: Hearts },
                Card { rank: Rank::Five, suit: Hearts },
                Card { rank: Rank::Seven, suit: Hearts },
                Card { rank: Rank::Nine, suit: Hearts },
            ];
        }
    let expected = hash_map! {
        PokerHand::Flush => 1,
    };
    test_hand_correctness(cards, &expected);
}

#[test]
fn test_simple_straight() {
    let cards: Vec<Card>;
    #[rustfmt::skip]
        {
            cards = vec![
                Card { rank: Rank::Ten, suit: Clubs },
                Card { rank: Rank::Jack, suit: Diamonds },
                Card { rank: Rank::Queen, suit: Hearts },
                Card { rank: Rank::King, suit: Spades },
                Card { rank: Rank::Ace, suit: Wild }, // Acting as Ace of any suit
            ];
        }
    let expected = hash_map! {
        PokerHand::Straight => 1,
    };
    test_hand_correctness(cards, &expected);
}

#[test]
fn test_with_multiple_wild_cards_forming_straight_flush() {
    let cards: Vec<Card>;
    #[rustfmt::skip]
        {
            cards = vec![
                Card { rank: Rank::Six, suit: Wild },
                Card { rank: Rank::Seven, suit: Wild },
                Card { rank: Rank::Eight, suit: Clubs },
                Card { rank: Rank::Eight, suit: Clubs },
                Card { rank: Rank::Nine, suit: Clubs },
                Card { rank: Rank::Ten, suit: Clubs },
            ];
        }
    let expected = hash_map! {
        PokerHand::StraightFlush => 1,
        PokerHand::Flush=>1,
        PokerHand::Pair => 1,
    };
    test_hand_correctness(cards, &expected);
}

#[test]
fn test_for_natural_full_house_with_single_wild() {
    let cards: Vec<Card>;
    #[rustfmt::skip]
        {
            cards = vec![
                Card { rank: Rank::Jack, suit: Diamonds },
                Card { rank: Rank::Jack, suit: Spades },
                Card { rank: Rank::Jack, suit: Clubs },
                Card { rank: Rank::Nine, suit: Hearts },
                Card { rank: Rank::Nine, suit: Wild }, // Acting as Nine of any suit
            ];
        }
    let expected = hash_map! {
        PokerHand::FullHouse => 1,
        PokerHand::Pair => 1,
        PokerHand::ThreeOfAKind => 1,
        PokerHand::TwoPair => 1,

    };
    test_hand_correctness(cards, &expected);
}
#[test]
fn test_flush_with_wild_cards_as_filler() {
    let cards: Vec<Card> = vec![
        Card {
            rank: Rank::Two,
            suit: Spades,
        },
        Card {
            rank: Rank::Four,
            suit: Spades,
        },
        Card {
            rank: Rank::Ace,
            suit: Wild,
        }, // Acting as Spades
        Card {
            rank: Rank::Eight,
            suit: Spades,
        },
        Card {
            rank: Rank::King,
            suit: Wild,
        }, // Acting as Spades
    ];
    let expected = hash_map! {
        PokerHand::Flush => 1,
    };
    test_hand_correctness(cards, &expected);
}
#[test]
fn test_higher_flush_with_wild_cards() {
    let cards: Vec<Card> = vec![
        Card {
            rank: Rank::Nine,
            suit: Hearts,
        },
        Card {
            rank: Rank::Ten,
            suit: Hearts,
        },
        Card {
            rank: Rank::Jack,
            suit: Hearts,
        },
        Card {
            rank: Rank::Queen,
            suit: Wild,
        },
        Card {
            rank: Rank::King,
            suit: Wild,
        },
        Card {
            rank: Rank::Ace,
            suit: Spades,
        },
    ];
    let expected = hash_map! {
        PokerHand::StraightFlush => 1,
        PokerHand::Straight => 1,
    };
    test_hand_correctness(cards, &expected);
}

#[test]
fn test_natural_full_house_vs_wild_full_house() {
    let cards: Vec<Card> = vec![
        Card {
            rank: Rank::Jack,
            suit: Diamonds,
        },
        Card {
            rank: Rank::Jack,
            suit: Spades,
        },
        Card {
            rank: Rank::Jack,
            suit: Clubs,
        },
        Card {
            rank: Rank::Nine,
            suit: Hearts,
        },
        Card {
            rank: Rank::Nine,
            suit: Wild,
        }, // Acting as Hearts for Full House
    ];
    let expected = hash_map! {
        PokerHand::FullHouse => 1,
        PokerHand::Pair => 1,
        PokerHand::ThreeOfAKind => 1,
        PokerHand::TwoPair => 1,
    };
    test_hand_correctness(cards, &expected);
}
