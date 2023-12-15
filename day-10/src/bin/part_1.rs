use std::{collections::HashMap, iter::successors};

use glam::IVec2;

fn main() {
    let input = include_str!("./sample_input.txt");
    let output = part_1(input);
    dbg!(output);
}

#[derive(Debug, PartialEq)]
enum PipeType {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    StartingPosition,
    Ground,
}

// #[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
// struct Position {
//     x: i32,
//     y: i32,
// }

// impl Position {
//     fn add(&Self, )
// }

#[derive(Debug, Eq, PartialEq, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
}

fn parse_input(input: &str) -> HashMap<IVec2, PipeType> {
    let mut map: HashMap<IVec2, _> = HashMap::new();

    for (row_index, line) in input.lines().enumerate() {
        for (col_index, col) in line.chars().enumerate() {
            let position = IVec2 {
                x: col_index as i32,
                y: row_index as i32,
            };
            // dbg!(row_index, col_index, col);
            let pipe_type = match col {
                'S' => PipeType::StartingPosition,
                'J' => PipeType::NorthWest,
                '|' => PipeType::Vertical,
                'L' => PipeType::NorthEast,
                '-' => PipeType::Horizontal,
                '7' => PipeType::SouthWest,
                'F' => PipeType::SouthEast,
                '.' => PipeType::Ground,
                _ => panic!("Should not happen"),
            };
            map.insert(position, pipe_type);
        }
    }

    map
}

fn part_1(input: &str) -> i32 {
    // println!("{:?}", input);
    let grid = parse_input(input);

    // find starting position
    let starting_position = grid
        .iter()
        .find_map(|(position, pipe_type)| {
            (pipe_type == &PipeType::StartingPosition).then_some(position)
        })
        .expect("there should be a starting position");

    // search all four sides
    let south = *starting_position + IVec2::new(0, 1);
    let south_position = grid
        .get(&south)
        .is_some_and(|pipe_type| match pipe_type {
            PipeType::Vertical | PipeType::NorthWest | PipeType::NorthEast => true,
            _ => false,
        })
        .then_some((Direction::North, south));

    let north = *starting_position + IVec2::new(0, -1);
    let north_position = grid
        .get(&north)
        .is_some_and(|pipe_type| match pipe_type {
            PipeType::Vertical | PipeType::SouthWest | PipeType::SouthEast => true,
            _ => false,
        })
        .then_some((Direction::South, north));

    let east = *starting_position + IVec2::new(1, 0);
    let east_position = grid
        .get(&east)
        .is_some_and(|pipe_type| match pipe_type {
            PipeType::Horizontal | PipeType::NorthWest | PipeType::SouthWest => true,
            _ => false,
        })
        .then_some((Direction::West, east));

    let west = *starting_position + IVec2::new(-1, 0);
    let west_position = grid
        .get(&west)
        .is_some_and(|pipe_type| match pipe_type {
            PipeType::Horizontal | PipeType::NorthEast | PipeType::SouthEast => true,
            _ => false,
        })
        .then_some((Direction::East, west));

    let mut iters = vec![south_position, north_position, west_position, east_position]
        .into_iter()
        .flatten()
        .map(|(direction, position)| {
            successors(
                Some((direction, position)),
                |(from_direction, curr_position)| {
                    let pipe_type = grid.get(curr_position).expect("should be a valid position");

                    let direction_to_go = match (from_direction, pipe_type) {
                        (Direction::North, PipeType::Vertical) => Direction::South,
                        (Direction::North, PipeType::NorthEast) => Direction::East,
                        (Direction::North, PipeType::NorthWest) => Direction::West,
                        (Direction::South, PipeType::Vertical) => Direction::North,
                        (Direction::South, PipeType::SouthEast) => Direction::East,
                        (Direction::South, PipeType::SouthWest) => Direction::West,
                        (Direction::East, PipeType::Horizontal) => Direction::West,
                        (Direction::East, PipeType::NorthEast) => Direction::North,
                        (Direction::East, PipeType::SouthEast) => Direction::South,
                        (Direction::West, PipeType::Horizontal) => Direction::East,
                        (Direction::West, PipeType::NorthWest) => Direction::North,
                        (Direction::West, PipeType::SouthWest) => Direction::South,
                        value => {
                            unreachable!(
                                "should not land on Ground or loop around again, {:?}",
                                value
                            );
                        }
                    };

                    dbg!(curr_position, from_direction, pipe_type, &direction_to_go);

                    Some(match direction_to_go {
                        Direction::North => (Direction::South, *curr_position + IVec2::new(0, -1)),
                        Direction::South => (Direction::North, *curr_position + IVec2::new(0, 1)),
                        Direction::East => (Direction::West, *curr_position + IVec2::new(1, 0)),
                        Direction::West => (Direction::East, *curr_position + IVec2::new(-1, 0)),
                    })
                },
            )
        });

    let path_a = iters.next().expect("path a should_exist");
    let path_b = iters.next().expect("path b should exist");
    let final_position = path_a
        .zip(path_b)
        .position(|(a, b)| a.1 == b.1)
        .expect("should meet in the middle");

    // println!("{:?}", iters);

    final_position as i32 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("./sample_input.txt");
        let output = part_1(input);
        assert_eq!(output, 8);
    }
}
