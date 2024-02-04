use std::{cmp::Ordering, collections::HashMap};

use aoc_2023_rs::camel_cards::*;

#[test]
fn hand_from_str_constructs_expected_hand() {
    let card_values = HashMap::from([
        ('A', 13),
        ('K', 12),
        ('Q', 11),
        ('J', 10),
        ('T', 9),
        ('9', 8),
        ('8', 7),
        ('7', 6),
        ('6', 5),
        ('5', 4),
        ('4', 3),
        ('3', 2),
        ('2', 1),
    ]);
    let expected = Hand {
        bid: 765,
        score: 1060888,
    };
    let actual = Hand::from_str("32T3K 765", card_values);

    assert_eq!(actual, expected);
}

#[test]
fn hand_cmp_returns_expected_values() {
    let card_values = HashMap::from([
        ('A', 13),
        ('K', 12),
        ('Q', 11),
        ('J', 10),
        ('T', 9),
        ('9', 8),
        ('8', 7),
        ('7', 6),
        ('6', 5),
        ('5', 4),
        ('4', 3),
        ('3', 2),
        ('2', 1),
    ]);
    let hand_a = Hand::from_str("32T3K 765", card_values.clone());
    let hand_b = Hand::from_str("T55J5 684", card_values.clone());

    let result = hand_a.cmp(&hand_b);
    assert_eq!(result, Ordering::Less);

    let result = hand_b.cmp(&hand_a);
    assert_eq!(result, Ordering::Greater);

    let result = hand_a.cmp(&hand_a);
    assert_eq!(result, Ordering::Equal);
}
