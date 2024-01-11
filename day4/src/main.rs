use std::{
    collections::{HashMap, VecDeque},
    fs,
};

use regex::Regex;

struct Card {
    id: usize,
    winning: Vec<u32>,
    current: Vec<u32>,
}

impl From<&str> for Card {
    fn from(value: &str) -> Self {
        let re = Regex::new(r"(?P<id>[\d]+):(?P<winning>[\d\s]+)\| (?P<current>[\d\s]+)").unwrap();
        let caps = re.captures(value).unwrap();
        let id: usize = caps["id"].parse().unwrap();
        let winning: Vec<u32> = caps["winning"]
            .split(' ')
            .filter(|line| !line.is_empty())
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        let current: Vec<u32> = caps["current"]
            .split(' ')
            .filter(|line| !line.is_empty())
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        Self {
            id,
            winning,
            current,
        }
    }
}

impl Card {
    fn get_matching_numbers(&self) -> usize {
        self.current
            .iter()
            .filter(|&num| self.winning.contains(num))
            .count()
    }

    fn get_next_cards(&self) -> Vec<usize> {
        let start = self.id + 1;
        (start..start + self.get_matching_numbers()).collect()
    }

    #[cfg(not(feature = "part2"))]
    fn get_points(&self) -> u32 {
        let count = self.get_matching_numbers() as u32;
        if count == 0 {
            return 0;
        }
        2u32.pow(count - 1)
    }
}

fn get_cards_count(cards: &[Card], id: usize, memo: &mut HashMap<usize, usize>) -> usize {
    if memo.contains_key(&id) {
        let ret = memo.get(&id).unwrap().to_owned();
        return ret;
    }

    let card = &cards[id - 1];
    let count = card.get_matching_numbers();

    if count == 0 {
        return 0;
    }

    let sum = card
        .get_next_cards()
        .iter()
        .map(|&e| 1 + get_cards_count(cards, e, memo))
        .sum::<usize>();

    memo.insert(id, sum);
    sum
}

fn main() {
    let input = fs::read_to_string("day4/input.txt").unwrap();
    let cards: Vec<Card> = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.into())
        .collect();

    #[cfg(not(feature = "part2"))]
    {
        let points: u32 = cards.iter().map(|c| c.get_points()).sum();
        println!("The total points are {points}");
    }
    #[cfg(feature = "part2")]
    {
        let mut memo: HashMap<usize, usize> = HashMap::new();
        let card_numbers: VecDeque<usize> = (1..cards.len() + 1).collect();

        let sum: usize = card_numbers
            .iter()
            .map(|&num| 1 + get_cards_count(&cards, num, &mut memo))
            .sum();

        println!("The total count of the scratchcards is {sum}");
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_get_cards_count() {
        let input = fs::read_to_string("input_ex.txt").unwrap();
        let cards: Vec<Card> = input
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|line| line.into())
            .collect();
        let mut memo: HashMap<usize, usize> = HashMap::new();
        let card_numbers: VecDeque<usize> = (1..cards.len() + 1).collect();
        let sum: usize = card_numbers
            .iter()
            .map(|&num| 1 + get_cards_count(&cards, num, &mut memo))
            .sum();
        println!("sum is {sum}");
        assert!(sum == 30);
    }
}
