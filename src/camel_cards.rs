use std::{cmp::Ordering, collections::HashMap};

#[derive(Debug, PartialEq)]
pub struct Hand {
    pub bid: u64,
    pub score: u64,
}

impl Hand {
    pub fn from_str(s: &str, card_values: &HashMap<char, u8>) -> Hand {
        let split_s: Vec<&str> = s.split(" ").collect();

        let bid: u64 = split_s.get(1).unwrap().parse().unwrap();
        let cards = split_s.get(0).unwrap();
        let score = calc_hand_score(*cards, card_values);

        Hand { bid, score }
    }

    /// Compares this card hand with another hand, based on their scores.
    pub fn cmp(&self, other: &Hand) -> Ordering {
        self.score.cmp(&other.score)
    }
}

fn calc_hand_score(cards: &str, card_values: &HashMap<char, u8>) -> u64 {
    let mut score = 0;
    let card_chars: Vec<char> = cards.chars().collect();
    let mut card_map: HashMap<char, u8> = HashMap::new();
    let total_cards: u64 = card_values.len() as u64;

    for (index, card) in card_chars.iter().enumerate() {
        let count: u8 = cards.matches(*card).count().try_into().unwrap();
        card_map.insert(*card, count);

        let card_position: u64 = (card_chars.len() - index - 1).try_into().unwrap();
        let card_val: u64 = *card_values.get(card).unwrap() as u64;
        score += card_position + (card_val * total_cards.pow(card_position as u32));
    }

    let max_score = (cards.len() - 1) + (card_values.len().pow(cards.len() as u32));
    let multiplier = max_score.to_string().len();

    score + calc_bonus_score(card_map) * 10_u64.pow(multiplier as u32)
}

fn calc_bonus_score(card_map: HashMap<char, u8>) -> u64 {
    let mut matches: Vec<u8> = card_map.values().cloned().collect();
    matches.sort_by(|a, b| b.cmp(a)); // sort in descending order

    match matches[0] {
        5 => return 6,
        4 => return 5,
        3 => match matches[1] {
            2 => return 4,
            1 => return 3,
            _ => panic!("Unable to calculate score for hand!"),
        },
        2 => match matches[1] {
            2 => return 2,
            1 => return 1,
            _ => panic!("Unable to calculate score for hand!"),
        },
        1 => return 0,
        _ => panic!("Unable to calculate score for hand!"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn calc_hand_score_returns_expected_results() {
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
        let score = calc_hand_score("32T3K", &card_values); // one pair, 1x multiplier
        assert_eq!(score, 1060888);

        let score = calc_hand_score("KK677", &card_values); // two pair, 2x multiplier
        assert_eq!(score, 2370035);
    }

    #[test]
    fn calc_bonus_score_returns_expected_results() {
        let card_map: HashMap<char, u8> = HashMap::from([('A', 5)]);
        assert_eq!(calc_bonus_score(card_map), 6);

        let card_map: HashMap<char, u8> = HashMap::from([('A', 4), ('K', 1)]);
        assert_eq!(calc_bonus_score(card_map), 5);

        let card_map: HashMap<char, u8> = HashMap::from([('A', 3), ('K', 2)]);
        assert_eq!(calc_bonus_score(card_map), 4);

        let card_map: HashMap<char, u8> = HashMap::from([('A', 3), ('K', 1), ('Q', 1)]);
        assert_eq!(calc_bonus_score(card_map), 3);

        let card_map: HashMap<char, u8> = HashMap::from([('A', 2), ('K', 2), ('Q', 1)]);
        assert_eq!(calc_bonus_score(card_map), 2);

        let card_map: HashMap<char, u8> = HashMap::from([('A', 2), ('K', 1), ('Q', 1), ('J', 1)]);
        assert_eq!(calc_bonus_score(card_map), 1);

        let card_map: HashMap<char, u8> =
            HashMap::from([('A', 1), ('K', 1), ('Q', 1), ('J', 1), ('T', 1)]);
        assert_eq!(calc_bonus_score(card_map), 0);
    }
}
