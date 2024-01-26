use std::collections::HashSet;

fn count_matches(line: &str) -> usize {
    let split_card_title: Vec<&str> = line.split(": ").collect();
    let split_winners: Vec<&str> = split_card_title[1].split(" | ").collect();

    let processed_winner_string = split_winners[0].replace("  ", " ");
    let winners: HashSet<&str> = processed_winner_string.split(" ").collect();
    let processed_actuals_string = split_winners[1].replace("  ", " ");
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
