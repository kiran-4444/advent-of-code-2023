use std::collections::HashMap;

use itertools::Itertools;
use nom::IResult;

fn main() {
    let input = include_str!("./input.txt");
    let output = part_1(input);
    dbg!(output.ok().unwrap().1);
}

#[derive(Debug, Clone, Copy)]
enum Type {
    Five = 6,
    Four = 5,
    Full = 4,
    Three = 3,
    Two = 2,
    One = 1,
    High = 0,
}

#[derive(Debug)]
struct Card<'a> {
    combination: &'a str,
    bid: i32,
    card_type: Type,
}

fn get_card_type(combination: &str) -> Type {
    let count = combination.chars().unique().count();
    if count == 1 {
        Type::Five
    } else if count == 5 {
        Type::High
    } else if count == 4 {
        Type::One
    } else {
        let mut map: HashMap<char, i32> = HashMap::new();

        for c in combination.chars() {
            let counter = map.entry(c).or_insert(0);
            *counter += 1;
        }

        let mut sorted_entries: Vec<_> = map.into_iter().map(|(c, v)| v).collect();
        sorted_entries.sort_by(|a, b| b.cmp(&a));

        // dbg!(sorted_entries);

        if sorted_entries.len() == 2 && sorted_entries[0] == 4 {
            Type::Four
        } else if sorted_entries.len() == 2 && sorted_entries[1] == 2 {
            Type::Full
        } else if sorted_entries.len() == 3
            && sorted_entries[0] == 3
            && sorted_entries[1] == 1
            && sorted_entries[2] == 1
        {
            Type::Three
        } else if sorted_entries.len() == 3
            && sorted_entries[0] == 2
            && sorted_entries[1] == 2
            && sorted_entries[2] == 1
        {
            Type::Two
        } else {
            println!("Should not happpen!");
            todo!()
        }
    }
}

fn part_1(input: &str) -> IResult<&str, String> {
    let mut cards = input
        .lines()
        .map(|line| {
            let splitted_line = line.split(" ").collect::<Vec<&str>>();
            Card {
                combination: splitted_line[0],
                bid: splitted_line[1].parse::<i32>().unwrap(),
                card_type: get_card_type(splitted_line[0]),
            }
        })
        .collect::<Vec<Card>>();

    let card_scores: Vec<(u32, u32, u32, u32, u32)> = cards
        .iter()
        .map(|card| {
            card.combination
                .chars()
                .map(|card| match card {
                    'A' => 14,
                    'K' => 13,
                    'Q' => 12,
                    'T' => 10,
                    'J' => 11,
                    value => value.to_digit(10).unwrap(),
                })
                .collect_tuple()
                .unwrap()
        })
        .collect();

    let sort_by_key = cards
        .iter()
        .zip(card_scores)
        .sorted_by_key(|x| (x.0.card_type as i32, x.1));

    dbg!(&sort_by_key);

    let ans = sort_by_key
        .enumerate()
        .map(|(index, (card, _))| {
            dbg!(index, card.bid);
            (index as i32 + 1) * card.bid
        })
        .sum::<i32>();

    dbg!(ans);

    Ok((input, ans.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("./sample_input.txt");
        let output = part_1(input).unwrap();
        assert_eq!(output.1, "6440");
    }
}
