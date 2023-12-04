use std::{cmp::max, collections::HashMap};

fn main() {
    let input = include_str!("./input.txt");
    let output = part_2(input);
    dbg!(output);
}

fn part_2(input: &str) -> u32 {
    let lines: Vec<&str> = input.split('\n').collect();

    let mut ans: u32 = 0;

    for (_index, line) in lines.iter().enumerate() {
        let sack: &str = line.split(": ").collect::<Vec<&str>>()[1];
        let mut total_cubes = HashMap::new();
        total_cubes.insert(String::from("red"), 0);
        total_cubes.insert(String::from("green"), 0);
        total_cubes.insert(String::from("blue"), 0);

        for segment in sack.split("; ").collect::<Vec<&str>>() {
            for cube in segment.split(", ").collect::<Vec<&str>>() {
                // dbg!(cube);
                let count_color: Vec<&str> = cube.split(" ").collect();
                let count = count_color[0].parse::<i32>().unwrap();
                let color = count_color[1];
                // dbg!(count, color);

                let max_color_count = total_cubes.get(color).unwrap();
                total_cubes.insert(color.to_string(), max(*max_color_count, count));
            }
        }

        let mut prod = 1;
        for (_color, count) in total_cubes.iter() {
            prod *= *count as u32;
        }

        ans += prod;
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2() {
        let input = include_str!("./input.txt");
        let output = part_2(input);
        assert_eq!(output, 2286);
    }
}
