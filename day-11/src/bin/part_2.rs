use itertools::Itertools;
use std::collections::{BTreeMap, HashMap};

fn main() {
    let input = include_str!("./input.txt");
    let output = part_2(input);
    dbg!(output);
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum RowOrCol {
    Row,
    Col,
}

fn make_grid(input: &str) -> (Vec<Vec<char>>, BTreeMap<(i64, RowOrCol), bool>) {
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut expansion_map: BTreeMap<(i64, RowOrCol), bool> = BTreeMap::new();

    for (index, line) in input.lines().enumerate() {
        grid.push(Vec::new());
        for character in line.chars() {
            grid[index].push(character);
        }
    }

    let row_to_insert = grid[0].iter().map(|_| '.').collect::<Vec<char>>();

    let mut new_grid: Vec<Vec<char>> = Vec::new();

    // expand rows
    for (index, row) in grid.clone().iter_mut().enumerate() {
        if row.iter().all(|character| *character == '.') {
            new_grid.push(row_to_insert.clone());
            // for i in 0..(1000000 - 1) {
            //     new_grid.push(row_to_insert.clone());
            // }
            expansion_map.insert((index as i64, RowOrCol::Row), true);
        } else {
            new_grid.push(row.clone());
        }
    }
    // print(&grid);
    grid = new_grid;

    let mut transposed: Vec<Vec<char>> = Vec::new();
    for (row_index, row) in grid.clone().iter().enumerate() {
        for (col_index, col) in row.iter().enumerate() {
            if transposed.len() <= col_index {
                transposed.push(Vec::new());
            }
            transposed[col_index].push(*col);
        }
    }

    // println!();
    // print(&transposed);

    let mut new_transposed: Vec<Vec<char>> = Vec::new();

    let col_to_insert = transposed[0].iter().map(|_| '.').collect::<Vec<char>>();
    for (index, col) in transposed.clone().iter().enumerate() {
        if col.iter().all(|character| *character == '.') {
            new_transposed.push(col_to_insert.clone());
            expansion_map.insert((index as i64, RowOrCol::Col), true);
        } else {
            new_transposed.push(col.clone());
        }
    }
    transposed = new_transposed;

    let mut grid: Vec<Vec<char>> = Vec::new();
    for (row_index, row) in transposed.clone().iter().enumerate() {
        for (col_index, col) in row.iter().enumerate() {
            if grid.len() <= col_index {
                grid.push(Vec::new());
            }
            grid[col_index].push(*col);
        }
    }

    (grid, expansion_map)
}

fn print(grid: &Vec<Vec<char>>) {
    for i in grid {
        for j in i {
            print!("{:?} ", *j);
        }
        println!();
    }
}

fn part_2(input: &str) -> i64 {
    let (grid, expansion_map) = make_grid(input);
    // print(&grid);
    dbg!(grid.len(), grid[0].len());

    let mut locations: Vec<(i64, i64)> = Vec::new();

    for (index, row) in grid.iter().enumerate() {
        for (col_index, col) in row.iter().enumerate() {
            if *col == '#' {
                locations.push((index as i64, col_index as i64));
            }
        }
    }

    // dbg!(&locations);
    dbg!(locations.len());

    let mut ans = 0;

    let mut pairs: Vec<((i64, i64), (i64, i64))> = Vec::new();

    for (x, y) in locations.clone() {
        for (a, b) in locations.clone() {
            if !pairs.contains(&((x, y), (a, b))) {
                let mut total_row_found = 0;
                let mut total_col_found = 0;
                for ((index, index_type), _) in expansion_map.iter() {
                    if *index_type == RowOrCol::Row {
                        if (x != *index && a != *index && x < *index && a > *index)
                            || (x <= *index && a > *index)
                            || (x < *index && a >= *index)
                            || (a <= *index && x > *index)
                            || (a < *index && x >= *index)
                        {
                            total_row_found += 1;
                        }
                    } else {
                        if (y != *index && b != *index && y < *index && b > *index)
                            || (y <= *index && b > *index)
                            || (y < *index && b >= *index)
                            || (b <= *index && y > *index)
                            || (b < *index && y >= *index)
                        {
                            total_col_found += 1;
                        }
                    }
                }

                let distance = (a - x).abs()
                    + (b - y).abs()
                    + total_row_found * (1000000 - 1)
                    + total_col_found * (1000000 - 1);
                pairs.push(((a, b), (x, y)));
                ans += distance;
            }
        }
    }

    dbg!(ans);

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2() {
        let input = include_str!("./sample_input.txt");
        let output = part_2(input);
        assert_eq!(output, 374);
    }
}
