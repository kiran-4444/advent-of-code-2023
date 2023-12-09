use itertools::Itertools;
use std::collections::{BTreeMap, HashMap};

fn main() {
    let input = include_str!("./input.txt");
    let output = part_1(input.to_string());
    dbg!(output);
}

fn get_differences(values: &Vec<i32>) -> Vec<i32> {
    let diff_vec: Vec<i32> = values
        .iter()
        .tuple_windows()
        .map(|(a, b)| *b - *a)
        .collect_vec();

    diff_vec.clone()
}

fn are_zeroes(sequence: &Vec<i32>) -> bool {
    sequence.iter().any(|digit| *digit != 0)
}

fn sequence(line: &str) -> i32 {
    let values = line
        .split(" ")
        .into_iter()
        .map(|value| value.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut sequence_values: Vec<Vec<i32>> = Vec::new();
    sequence_values.push(values.clone());

    let mut curr_sequence = values.clone();

    while are_zeroes(&curr_sequence) {
        curr_sequence = get_differences(&curr_sequence);
        // dbg!(&curr_sequence);
        sequence_values.push(curr_sequence.clone());
    }

    // dbg!(sequence_values);
    // println!("{:?}", sequence_values);
    let ans = sequence_values
        .iter()
        .map(|sequence| sequence.get(sequence.len() - 1).unwrap())
        .sum::<i32>();
    ans
}

fn part_1(input: String) -> i32 {
    let ans = input.lines().map(|line| sequence(line)).sum::<i32>();
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("./sample_input.txt");
        let output = part_1(input.to_string());
        assert_eq!(output, 114);
    }
}
