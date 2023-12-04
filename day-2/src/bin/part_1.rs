use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = part_1(input);
    dbg!(output);
}

fn part_1(input: &str) -> i32 {
    let lines: Vec<&str> = input.split('\n').collect();
    let mut total_cubes = HashMap::new();
    total_cubes.insert(String::from("red"), 12);
    total_cubes.insert(String::from("green"), 13);
    total_cubes.insert(String::from("blue"), 14);

    let mut ans: i32 = 0;

    for (index, line) in lines.iter().enumerate() {
        let sack: &str = line.split(": ").collect::<Vec<&str>>()[1];
        let mut is_possible = true;
        for segment in sack.split("; ").collect::<Vec<&str>>() {
            for cube in segment.split(", ").collect::<Vec<&str>>() {
                // dbg!(cube);
                let count_color: Vec<&str> = cube.split(" ").collect();
                let count = count_color[0].parse::<u32>().unwrap();
                let color = count_color[1];
                // dbg!(count, color);

                let max_color_count = total_cubes.get(color).unwrap();
                if *max_color_count < count {
                    is_possible = false;
                    break;
                }
            }
        }
        if is_possible {
            // dbg!(index);
            ans += index as i32 + 1;
        }
        // println!();
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("./input.txt");
        let output = part_1(input);
        assert_eq!(output, 8);
    }
}
