use core::num;
use itertools::Itertools;
use std::{
    collections::{BTreeMap, HashMap},
    vec,
};

fn main() {
    let input = include_str!("./input.txt");
    let output = part_2(input);
    dbg!(output);
}

#[derive(Debug)]
enum Value {
    Symbol(char),
    Empty,
    Number(u32),
}

fn part_2(input: &str) -> u32 {
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(row_index, row)| {
            row.chars()
                .enumerate()
                .map(move |(column_index, character)| {
                    (
                        (row_index as i32, column_index as i32),
                        match character {
                            '.' => Value::Empty,
                            c if c.is_ascii_digit() => Value::Number(c.to_digit(10).unwrap()),
                            c => Value::Symbol(c),
                        },
                    )
                })
        })
        .collect::<BTreeMap<(i32, i32), Value>>();

    let mut numbers: Vec<Vec<((i32, i32), u32)>> = vec![];
    for ((row_index, column_index), value) in map.iter() {
        if let Value::Number(num) = value {
            match numbers.iter().last() {
                Some(v) => {
                    let last_num = v.iter().last().unwrap();
                    let last_num_col = last_num.0 .1;
                    if last_num_col + 1 == *column_index {
                        let last = numbers.iter_mut().last().unwrap();
                        last.push(((*row_index, *column_index), *num));
                    } else {
                        numbers.push(vec![((*row_index, *column_index), *num)]);
                    }
                }
                None => {
                    numbers.push(vec![((*row_index, *column_index), *num)]);
                }
            }
        }
    }

    let mut ans = 0;
    let mut doubles: HashMap<(i32, i32), (i32, Vec<i32>)> = HashMap::new();
    for num_list in numbers {
        let positions = [
            (1, 0),
            (-1, 0),
            (0, 1),
            (0, -1),
            (1, 1),
            (-1, -1),
            (1, -1),
            (-1, 1),
        ];
        let number_positions: Vec<(i32, i32)> =
            num_list.iter().map(|((x, y), _)| (*x, *y)).collect();
        let positions_to_check: Vec<(i32, i32)> = num_list
            .iter()
            .flat_map(|((x, y), _)| positions.iter().map(move |(dx, dy)| (dx + x, dy + y)))
            .unique()
            .filter(|num| !number_positions.contains(num))
            .collect();

        positions_to_check.iter().any(|pos| {
            let value = map.get(pos);
            if let Some(Value::Symbol('*')) = value {
                println!("{:?}", pos);
                if doubles.contains_key(pos) {
                    let curr_val = doubles.get(pos).unwrap();
                    let num = num_list
                        .iter()
                        .map(|(_, num)| num.to_string())
                        .collect::<String>()
                        .parse::<i32>()
                        .unwrap();

                    let mut curr_num_list = (*curr_val).1.clone();
                    curr_num_list.push(num);
                    doubles.insert(*pos, ((*curr_val).0 + 1, curr_num_list));
                } else {
                    let num = num_list
                        .iter()
                        .map(|(_, num)| num.to_string())
                        .collect::<String>()
                        .parse::<i32>()
                        .unwrap();

                    let mut new_num_list = Vec::new();
                    new_num_list.push(num);
                    doubles.insert(*pos, (1, new_num_list));
                }
                true
            } else {
                false
            }
        });
    }

    println!("{:?}", doubles);

    for (_pos, (count, nums)) in doubles {
        if count == 2 {
            ans += nums.iter().product::<i32>();
        }
    }

    println!("{:?}", ans);

    ans as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2() {
        let input = include_str!("./input.txt");
        let output = part_2(input);
        assert_eq!(output, 467835);
    }
}

// #[allow(unused_comparisons)]
// fn main() {
//     let input = include_str!("./sample_input.txt");
//     let output = part_1(input);
//     dbg!(output);
// }

// fn part_1(input: &str) -> i32 {
//     let matrix: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
//     let mut matrix_2: Vec<Vec<String>> = Vec::new();

//     for (_outer_index, i) in matrix.iter().enumerate() {
//         let acc = Vec::new();
//         let tmp = String::new();
//         let row = i
//             .iter()
//             .fold((acc, tmp), |(mut acc, tmp), x| {
//                 if x.is_digit(10) {
//                     let mut tmp_size = [0; 4];
//                     if tmp != "" {
//                         (acc, tmp + (*x as char).encode_utf8(&mut tmp_size))
//                     } else {
//                         (acc, (*x as char).encode_utf8(&mut tmp_size).to_string())
//                     }
//                 } else {
//                     let mut tmp_len = tmp.len();
//                     if tmp_len >= 1 {
//                         tmp_len -= 1;
//                     }
//                     acc.push(tmp);
//                     for _k in 0..tmp_len {
//                         acc.push("0".to_string());
//                     }
//                     if x.to_string() != "" {
//                         acc.push(x.to_string());
//                     }
//                     (acc, "".to_string())
//                 }
//             })
//             .0;
//         let row: Vec<String> = row
//             .iter()
//             .filter(|x| !x.is_empty())
//             .map(|x| x.to_string())
//             .collect();
//         matrix_2.push(row);
//     }

//     dbg!(&matrix_2);

//     // dbg!(matrix_2);
//     // println!("matrix_2: {:?}", matrix_2);
//     let mut ans: i32 = 0;
//     for (outer_index, row) in matrix_2.iter().enumerate() {
//         for (inner_index, column) in row.iter().enumerate() {
//             let is_num = column.parse::<i32>().is_ok();
//             if is_num {
//                 let num = column.parse::<i32>().unwrap();

//                 // check adjacent columns for anything other than a .
//                 // checking right character
//                 if inner_index + 1 < row.len() && row[inner_index + 1] != ".".to_string() {
//                     ans += ans + num;
//                     continue;
//                 }
//                 if inner_index > 0 && row[inner_index - 1] != ".".to_string() {
//                     ans += ans + num;
//                     continue;
//                 }

//                 // checking adjacent rows

//             }
//         }
//     }

//     dbg!(ans);

//     todo!()
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_part_1() {
//         let input = include_str!("./sample_input.txt");
//         let output = part_1(input);
//         assert_eq!(output, 4361);
//     }
// }
