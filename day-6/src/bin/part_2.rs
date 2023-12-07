use std::iter::zip;

use itertools::Itertools;
use nom::IResult;

fn trim_extra_spaces(input: &str) -> String {
    input.trim().split(' ').filter(|s| !s.is_empty()).join(" ")
}
fn main() {
    let input = include_str!("./input.txt");
    let output = part_2(input);
    dbg!(output.ok().unwrap().1);
}

fn part_2(input: &str) -> IResult<&str, String> {
    let binding = trim_extra_spaces(input);
    let input_new = binding.as_str();
    let mut lines = input_new.split("\n").collect::<Vec<&str>>();
    // dbg!(input_new);
    // dbg!(&lines);
    lines[0] = lines[0].split("Time: ").collect::<Vec<&str>>()[1];
    lines[1] = lines[1].split("Distance: ").collect::<Vec<&str>>()[1];

    let times = lines[0]
        .split(" ")
        .map(|d| d.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let highest_distances = lines[1]
        .split(" ")
        .map(|d| d.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    // dbg!(times, highest_distances);

    let mut ans = 1;
    for (time, highest_distance) in zip(times, highest_distances) {
        let mut distances: Vec<u64> = Vec::new();
        for i in 0..time {
            let curr_dist = (time - i) * i;

            if curr_dist > highest_distance {
                distances.push(curr_dist);
            }
        }
        ans = ans * distances.len();
    }
    Ok((input, ans.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2() {
        let input = include_str!("./input.txt");
        let output = part_2(input).unwrap();
        assert_eq!(output.1, "288");
    }
}
