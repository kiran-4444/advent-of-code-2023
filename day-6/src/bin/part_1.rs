use std::iter::zip;

use itertools::Itertools;
use nom::IResult;

fn trim_extra_spaces(input: &str) -> String {
    input.trim().split(' ').filter(|s| !s.is_empty()).join(" ")
}
fn main() {
    let input = include_str!("./sample_input.txt");
    let output = part_1(input);
    dbg!(output.ok().unwrap().1);
}

fn part_1(input: &str) -> IResult<&str, String> {
    let binding = trim_extra_spaces(input);
    let input_new = binding.as_str();
    let mut lines = input_new.split("\n").collect::<Vec<&str>>();
    // dbg!(input_new);
    // dbg!(&lines);
    lines[0] = lines[0].split("Time: ").collect::<Vec<&str>>()[1];
    lines[1] = lines[1].split("Distance: ").collect::<Vec<&str>>()[1];

    let times = lines[0]
        .split(" ")
        .map(|d| d.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let highest_distances = lines[1]
        .split(" ")
        .map(|d| d.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    // dbg!(times, highest_distances);

    let mut ans = 1;
    for (time, highest_distance) in zip(times, highest_distances) {
        let mut distances: Vec<i32> = Vec::new();
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
    fn test_part_1() {
        let input = include_str!("./sample_input.txt");
        let output = part_1(input).unwrap();
        assert_eq!(output.1, "288");
    }
}
