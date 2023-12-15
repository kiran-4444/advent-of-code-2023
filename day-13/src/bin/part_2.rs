fn main() {
    let input = include_str!("./input.txt");
    let output = part_2(input);
    dbg!(output);
}

fn print(grid: &Vec<Vec<char>>) {
    for i in grid {
        for j in i {
            print!("{:?} ", *j);
        }
        println!();
    }
}

fn transpose(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let row_count = matrix.len();
    let col_count = matrix[0].len();

    let mut transposed = vec![vec![' '; row_count]; col_count];
    for (i, row) in matrix.iter().enumerate() {
        for (j, &val) in row.iter().enumerate() {
            transposed[j][i] = val;
        }
    }

    transposed
}

fn get_reflection_index(matrix: &[Vec<char>]) -> usize {
    for i in 1..matrix.len() {
        let mut is_smudged = false;
        let mut is_reflection = true;

        for j in 1..=i {
            if i + j > matrix.len() {
                break;
            }
            if matrix[i - j] != matrix[i + j - 1] {
                if !is_smudged && is_one_off(&matrix[i - j], &matrix[i + j - 1]) {
                    is_smudged = true;
                } else {
                    is_reflection = false;
                    break;
                }
            }
        }
        if is_reflection && is_smudged {
            return i;
        }
    }
    0
}

fn is_one_off(row_1: &[char], row_2: &[char]) -> bool {
    let mut one_off = false;

    for i in 0..row_1.len() {
        if row_1[i] != row_2[i] {
            if one_off {
                return false;
            }
            one_off = true;
        }
    }
    one_off
}

fn part_2(input: &str) -> usize {
    let mut matrix: Vec<Vec<char>> = Vec::new();
    let mut matrices: Vec<Vec<Vec<char>>> = Vec::new();

    for line in input.lines() {
        if line.is_empty() {
            matrices.push(matrix);
            matrix = Vec::new();
        } else {
            matrix.push(line.chars().collect::<Vec<char>>());
        }
    }
    matrices.push(matrix);
    // dbg!(matrices.len(), matrices[0].len());
    let mut total: usize = 0;

    for matrix in matrices {
        let transposed = transpose(&matrix);
        let vertical = get_reflection_index(&transposed);
        // dbg!(vertical);

        // print(&matrix);

        // match vertical {
        //     0 => {
        //         let horizontal = get_reflection_index(&matrix);
        //         total += horizontal * 100
        //     }
        //     _ => {
        //         total += vertical;
        //     }
        // };
        if vertical == 0 {
            // Also need to check horizontal
            let horizontal = get_reflection_index(&matrix);
            total += horizontal * 100;
        } else {
            total += vertical;
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2() {
        let input = include_str!("./sample_input.txt");
        let output = part_2(input);
        assert_eq!(output, 405);
    }
}
