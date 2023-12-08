use std::collections::{BTreeMap, HashMap};

use itertools::Itertools;
use winnow::combinator::{delimited, preceded, separated, separated_pair};
use winnow::prelude::*;
use winnow::token::take_while;
use winnow::Parser;

fn main() {
    let input = include_str!("./input.txt");
    let output = part_2(input.to_string());
    dbg!(output);
}

fn parse_chars<'s>(input: &mut &'s str) -> PResult<&'s str> {
    take_while(1.., ('a'..='z', 'A'..='Z', '1'..='9', '0')).parse_next(input)
}

fn parse_newlines<'s>(input: &mut &'s str) -> PResult<&'s str> {
    take_while(0.., '\n').parse_next(input)
}

fn parse_special_chars<'s>(input: &mut &'s str) -> PResult<&'s str> {
    take_while(1.., (' ', '=')).parse_next(input)
}

fn parse_separated(s: &str) -> IResult<&str, Vec<&str>> {
    separated(
        0..,
        delimited(
            '(',
            take_while(0.., ('A'..='Z', ',', ' ', '1'..='9', '0')),
            ')',
        ),
        ",",
    )
    .parse_peek(s)
}

fn parse_input<'s>(input: &mut &'s str) -> (&'s str, (&'s str, &'s str), &'s str) {
    let (remaining, source) = preceded(parse_newlines, parse_chars)
        .parse_peek(input)
        .unwrap(); // parse source (AAA)

    let (remaining, _) = parse_special_chars.parse_peek(remaining).unwrap(); // parse till destination tuple ((DDD, EEE....))

    let (remaining, result) = parse_separated(remaining).unwrap(); // parse destination as list (["DDD, EEE"])

    let (_, destinations) = separated_pair(parse_chars, ", ", parse_chars)
        .parse_peek(result[0])
        .unwrap(); // parse string list in list of destinations (["DDD", "EEE"])

    (source, destinations, remaining)
}

fn parse_inputs<'s>(input: &mut &'s str) -> (&'s str, Vec<(&'s str, (&'s str, &'s str))>) {
    let (remaining, result) = parse_chars.parse_peek(input).unwrap(); // parse directions

    *input = remaining;
    let mut parsed = Vec::new();
    while !input.is_empty() {
        let (source, destinations, remaining) = parse_input(input);
        parsed.push((source, (destinations)));
        *input = remaining;
    }
    (result, parsed)
}

fn is_all_destinations(curr_sources: &Vec<&str>) -> bool {
    (*curr_sources).iter().all(|source| source.ends_with('Z'))
}

fn cycle(
    source: &str,
    directions_list: Vec<char>,
    source_dest_map: BTreeMap<&str, (&str, &str)>,
    destinations: Vec<&str>,
) -> i64 {
    let mut curr_source = source;
    let mut ans = 0;
    let mut directions_list = directions_list.clone();

    while !destinations.contains(&curr_source) {
        if directions_list[ans] == 'L' {
            let curr_dest = source_dest_map.get(curr_source).unwrap().0;
            curr_source = curr_dest;
            ans += 1;
        } else {
            let curr_dest = source_dest_map.get(curr_source).unwrap().1;
            curr_source = curr_dest;
            ans += 1;
        }

        if directions_list.len() - 1 == ans && !destinations.contains(&curr_source) {
            // extend directions_list
            let mut new_directions_list = directions_list.clone();
            for c in directions_list {
                new_directions_list.push(c);
            }
            directions_list = new_directions_list;
        }
    }
    ans as i64
}

// fn walk(
//     directions: &str,
//     source_dest_map: BTreeMap<&str, (&str, &str)>,
//     sources: Vec<&str>,
// ) -> i64 {
//     let mut sources = sources.clone();
//     let mut ans = 0;
//     let mut directions_list = directions.chars().into_iter().collect::<Vec<char>>();

//     while !is_all_destinations(&sources) {
//         if directions_list[ans] == 'L' {
//             for (index, i) in sources.clone().iter().enumerate() {
//                 let curr_dest = source_dest_map.get(i).unwrap().0;
//                 sources[index] = curr_dest;
//             }
//         } else {
//             for (index, i) in sources.clone().iter().enumerate() {
//                 let curr_dest = source_dest_map.get(i).unwrap().1;
//                 sources[index] = curr_dest;
//             }
//         }

//         if directions_list.len() - 1 == ans && !is_all_destinations(&sources) {
//             // extend directions_list
//             let mut new_directions_list = directions_list.clone();
//             for c in directions_list {
//                 new_directions_list.push(c);
//             }
//             directions_list = new_directions_list;
//         }
//         ans += 1;

//         // dbg!(&sources, &directions_list);
//     }

//     dbg!(ans);
//     ans as i64
// }

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}

fn lcm(nums: &[i64]) -> i64 {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd(a, b)
}

fn part_2(input: String) -> i64 {
    let (directions, source_destination_pairs) = parse_inputs(&mut input.as_str());

    // walk!
    // get all the sources ending with A
    let sources: Vec<&str> = source_destination_pairs
        .iter()
        .filter(|(src, _)| src.ends_with('A'))
        .map(|(src, _)| *src)
        .collect();

    let destinations: Vec<&str> = source_destination_pairs
        .iter()
        .filter(|(src, _)| src.ends_with('Z'))
        .map(|(src, _)| *src)
        .collect();

    let source_dest_map = source_destination_pairs
        .iter()
        .map(|(src, dest)| (*src, *dest))
        .collect::<BTreeMap<&str, (&str, &str)>>();

    let ans = sources
        .iter()
        .map(|source| {
            let result = cycle(
                source,
                directions.chars().into_iter().collect_vec(),
                source_dest_map.clone(),
                destinations.clone(),
            );
            result
        })
        .collect::<Vec<i64>>();

    let ans = lcm(&ans);

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2() {
        let input = include_str!("./sample_input.txt");
        let output = part_2(input.to_string());
        assert_eq!(output, 6);
    }
}
