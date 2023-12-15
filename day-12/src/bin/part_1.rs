use itertools::Itertools;
use std::collections::{BTreeMap, HashMap};

fn main() {
    let input = include_str!("./input.txt");
    let output = part_1(input);
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

fn get_count(
    memo: &mut BTreeMap<(usize, usize, usize), usize>,
    line: Vec<char>,
    counts: Vec<usize>,
    pos: usize,
    curr_count: usize,
    count_pos: usize,
) -> usize {
    let key = (pos, curr_count, count_pos);
    if memo.contains_key(&key) {
        let ret = *memo.get(&key).unwrap();
        return ret;
    }
    if pos == line.len() as usize {
        if counts.len() as usize == count_pos {
            1
        } else {
            0
        }
    } else if line[pos as usize] == '#' {
        let ret = get_count(memo, line, counts, pos + 1, curr_count + 1, count_pos);
        ret
    } else if line[pos as usize] == '.' || count_pos == counts.len() as usize {
        if count_pos < counts.len() as usize && curr_count == counts[count_pos as usize] {
            let ret = get_count(memo, line, counts, pos + 1, 0, count_pos + 1);
            ret
        } else if curr_count == 0 {
            let ret = get_count(memo, line, counts, pos + 1, 0, count_pos);
            ret
        } else {
            0
        }
    } else {
        let hash_count = get_count(
            memo,
            line.clone(),
            counts.clone(),
            pos + 1,
            curr_count + 1,
            count_pos,
        );
        let mut dot_count = 0;
        if curr_count == counts[count_pos as usize] {
            dot_count = get_count(memo, line.clone(), counts, pos + 1, 0, count_pos + 1);
        } else if curr_count == 0 {
            dot_count = get_count(memo, line.clone(), counts, pos + 1, 0, count_pos);
        }
        let ret = hash_count + dot_count;
        memo.insert(key, ret);
        ret
    }
}

fn part_1(input: &str) -> usize {
    let data = input
        .lines()
        .map(|line| line.split(" ").collect_vec())
        .collect::<Vec<Vec<&str>>>();

    let mut ans = 0;
    for line in data {
        let counts = line[1]
            .split(",")
            .collect_vec()
            .iter()
            .map(|digits| digits.parse::<usize>().unwrap())
            .collect_vec();
        let mut map: BTreeMap<(usize, usize, usize), usize> = BTreeMap::new();
        let mut curr_line = line[0].chars().collect_vec();
        curr_line.push('.');
        ans += get_count(&mut map, curr_line, counts, 0, 0, 0);
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("./sample_input.txt");
        let output = part_1(input);
        assert_eq!(output, 21);
    }
}
