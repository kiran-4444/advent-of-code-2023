use itertools::Itertools;
use std::collections::{BTreeMap, HashMap};

fn main() {
    let input = include_str!("./input.txt");
    let output = part_2(input.to_string());
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

    for (row_index, row) in sequence_values.clone().iter().enumerate().rev() {
        if row_index == sequence_values.len() - 1 {
            sequence_values[row_index].insert(0, 0);
        } else {
            let val_to_insert = sequence_values[row_index].get(0).unwrap()
                - sequence_values[row_index + 1].get(0).unwrap();
            sequence_values[row_index].insert(0, val_to_insert);
            // dbg!(sequence_values[row_index].get(0).unwrap());
            // dbg!(sequence_values[row_index + 1].get(0).unwrap());
            // dbg!(val_to_insert);
        }
    }

    // dbg!(sequence_values);
    // println!("{:?}", sequence_values);
    // let vec = sequence_values
    //     .iter()
    //     .map(|sequence| sequence.get(0).unwrap())
    //     .collect_vec();

    // dbg!(&vec);

    // let mut ans: i32 = 0;

    // for i in vec {
    //     ans = i - ans;
    // }

    sequence_values[0][0]
}

fn part_2(input: String) -> i32 {
    let ans = input.lines().map(|line| sequence(line)).sum::<i32>();
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2() {
        let input = include_str!("./sample_input.txt");
        let output = part_2(input.to_string());
        assert_eq!(output, 2);
    }
}
