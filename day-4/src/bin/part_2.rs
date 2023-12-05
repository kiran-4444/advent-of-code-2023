use std::collections::BTreeMap;

use itertools::Itertools;
use nom::{bytes::complete::tag, character::complete::space0, sequence::preceded, IResult};

fn main() {
    let input = include_str!("./input.txt");
    let trimmed_input = trim_extra_spaces(input);
    // println!("{:?}", trimmed_input);
    let output = part_2(trimmed_input.as_str());
    dbg!(&output.unwrap().1);
}

fn trim_extra_spaces(input: &str) -> String {
    input.trim().split(' ').filter(|s| !s.is_empty()).join(" ")
}

fn part_2(input: &str) -> IResult<&str, i32> {
    let mut copies: BTreeMap<i32, (i32, i32)> = BTreeMap::new();

    for (index, line) in input.lines().enumerate() {
        let (line_details, _) = preceded(tag("Card "), space0)(line)?;
        // dbg!(line_details);

        let remaining = line_details.split(": ").collect::<Vec<&str>>()[1];
        // dbg!(remaining);
        let cards = remaining.split(" | ").collect::<Vec<&str>>();
        let winning = cards[0]
            .trim()
            .split(' ')
            .filter(|s| !s.is_empty())
            .join(" ");
        let available = cards[1]
            .trim()
            .split(' ')
            .filter(|s| !s.is_empty())
            .join(" ");
        // dbg!(&winning, &available);
        let winning = winning
            .split(" ")
            .into_iter()
            .map(|digit| digit.trim().parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let available = available
            .split(" ")
            .into_iter()
            .map(|digit| digit.trim().parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let mut temp_ans = 0;

        for i in winning {
            if available.contains(&i) {
                temp_ans += 1;
            }
        }

        copies.insert(index as i32, (1, temp_ans));
    }

    let copies_len: i32 = copies.len() as i32;

    for i in 0..copies_len {
        let increment = copies.get(&i).unwrap().1;
        let to_add = copies.get(&i).unwrap().0;
        for j in (i + 1)..(i + 1 + increment) {
            copies.entry(j).and_modify(|value| (*value).0 += to_add);
        }
    }
    dbg!(&copies);

    let ans = copies.iter().map(|(_, (val, _))| *val).sum();

    Ok((input, ans))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2() {
        let input = include_str!("./input.txt");
        let output = part_2(input).unwrap();
        assert_eq!(output.1, 30);
    }
}
