use itertools::Itertools;
use std::collections::{BTreeMap, HashMap};

fn main() {
    let input = include_str!("./sample_input.txt");
    let output = part_1(input);
    dbg!(output);
}

fn make_grid(input: &str) -> Vec<Vec<char>> {
    let mut grid: Vec<Vec<char>> = Vec::new();

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
            // new_grid.push(row_to_insert.clone());
            for _ in 0..9 {
                new_grid.push(row_to_insert.clone());
            }
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
            // println!("Inserting at index: {:?}", index);
            // transposed.insert(index + 1, col_to_insert.clone());
            new_transposed.push(col_to_insert.clone());
            // new_transposed.push(col_to_insert.clone());
            for _ in 0..9 {
                new_transposed.push(col_to_insert.clone());
            }
        } else {
            new_transposed.push(col.clone());
        }
    }
    // println!();
    // print(&new_transposed);
    // println!();
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

    grid
}

fn print(grid: &Vec<Vec<char>>) {
    for i in grid {
        for j in i {
            print!("{:?} ", *j);
        }
        println!();
    }
}

fn part_1(input: &str) -> i32 {
    let grid = make_grid(input);
    print(&grid);
    dbg!(grid.len(), grid[0].len());

    let mut locations: Vec<(i32, i32)> = Vec::new();

    for (index, row) in grid.iter().enumerate() {
        for (col_index, col) in row.iter().enumerate() {
            if *col == '#' {
                locations.push((index as i32, col_index as i32));
            }
        }
    }

    dbg!(&locations);
    dbg!(locations.len());

    let mut ans = 0;

    let mut pairs: Vec<((i32, i32), (i32, i32))> = Vec::new();

    for (x, y) in locations.clone() {
        for (a, b) in locations.clone() {
            if !pairs.contains(&((x, y), (a, b))) {
                let distance = (a - x).abs() + (b - y).abs();
                pairs.push(((a, b), (x, y)));
                dbg!(((a, b), (x, y)), distance);
                println!();
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
    fn test_part_1() {
        let input = include_str!("./sample_input.txt");
        let output = part_1(input);
        assert_eq!(output, 374);
    }
}
