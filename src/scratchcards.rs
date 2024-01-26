use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader},
};

/// Performs the algorithm which solves the Part 1 problem
pub fn sum_total_winnings(path: &str) -> usize {
    let mut sum = 0;

    let file = File::open(path).unwrap();
    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        let matches = count_matches(line.as_str());
        if matches == 0 {
            continue;
        }
        sum += 2usize.pow(matches as u32 - 1);
    }

    sum
}

/// Performs the algorithm which solves the Part 2 problem
pub fn sum_total_scratchcards(path: &str) -> usize {
    let mut sum = 0;
    let mut queue: VecDeque<usize> = VecDeque::new();

    let file = File::open(path).unwrap();
    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        let curr_cards = match queue.pop_front() {
            Some(i) => i + 1,
            None => 1,
        };
        sum += curr_cards;
        let matches = count_matches(line.as_str());
        let mut count = 0;

        while count < matches {
            if let Some(e) = queue.get_mut(count) {
                *e += curr_cards;
            } else {
                queue.push_back(curr_cards);
            }
            count += 1;
        }
    }

    sum
}

fn count_matches(line: &str) -> usize {
    let split_card_title: Vec<&str> = line.split(": ").collect();
    let split_winners: Vec<&str> = split_card_title[1].split(" | ").collect();

    let processed_winner_string = split_winners[0].trim().replace("  ", " ");
    let winners: HashSet<&str> = processed_winner_string.split(" ").collect();
    let processed_actuals_string = split_winners[1].trim().replace("  ", " ");
    let actuals: HashSet<&str> = processed_actuals_string.split(" ").collect();

    let intersection: HashSet<_> = winners.intersection(&actuals).collect();

    intersection.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_matches_returns_correct_number() {
        let result = count_matches("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53");
        assert_eq!(result, 4);
    }
}
