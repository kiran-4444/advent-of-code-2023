use std::fs;

fn main() {
    let input = include_str!("./input.txt");
    let output = part_2(input);
    dbg!(output);
}

#[allow(unused_variables)]
fn part_2(input: &str) -> i32 {
    let data: Vec<&str> = input.split("\n").collect();
    let mut final_ans: i32 = 0;
    let mut temp_num: i32 = 0;
    let digits_vec: Vec<&str> = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let mut temp_num_vec: Vec<String> = Vec::new();
    for i in data {
        let curr_str = String::from(i);
        let mut curr_str_clone = curr_str.clone();
        for curr_index in 0..curr_str_clone.len() {
            for digit in digits_vec.iter() {
                if curr_index + digit.len() > curr_str_clone.len() {
                    continue;
                }
                if curr_str_clone[curr_index..].starts_with(digit) {
                    curr_str_clone.replace_range(
                        curr_index..curr_index + digit.len(),
                        digits_vec
                            .iter()
                            .position(|&r| r == *digit)
                            .unwrap()
                            .to_string()
                            .as_str(),
                    );
                    break;
                }
            }
        }
        for j in curr_str_clone.chars() {
            if j.is_digit(10) {
                temp_num = j.to_digit(10).unwrap() as i32;
                break;
            }
        }
        let curr_str = String::from(i);
        let mut curr_str_clone = curr_str.clone();
        for curr_index in (0..curr_str_clone.len()).rev() {
            for digit in digits_vec.iter() {
                if curr_index + digit.len() > curr_str_clone.len() {
                    continue;
                }
                if curr_str_clone[curr_index..].starts_with(digit) {
                    curr_str_clone.replace_range(
                        curr_index..curr_index + digit.len(),
                        digits_vec
                            .iter()
                            .position(|&r| r == *digit)
                            .unwrap()
                            .to_string()
                            .as_str(),
                    );
                    break;
                }
            }
        }
        println!("{:?}", i);
        println!("{:?}", curr_str_clone);

        for j in curr_str_clone.chars().rev() {
            if j.is_digit(10) {
                temp_num = temp_num * 10 + j.to_digit(10).unwrap() as i32;
                break;
            }
        }
        final_ans += temp_num;
        println!("{:?}", temp_num);
        temp_num_vec.push(temp_num.to_string());
        temp_num = 0;
    }
    println!("{:?}", final_ans);
    final_ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2() {
        // let input = include_str!("./sample_input.txt");
        let input = "5eight82sixtwonev";
        let output = part_2(input);
        assert_eq!(output, 51);
    }
}
