use std::collections::{BTreeMap, HashMap};

use winnow::combinator::{delimited, preceded, separated, separated_pair};
use winnow::prelude::*;
use winnow::token::take_while;
use winnow::Parser;

fn main() {
    let input = include_str!("./input.txt");
    let output = part_1(input.to_string());
    dbg!(output);
}

fn parse_chars<'s>(input: &mut &'s str) -> PResult<&'s str> {
    take_while(1.., ('a'..='z', 'A'..='Z')).parse_next(input)
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
        delimited('(', take_while(0.., ('A'..='Z', ',', ' ')), ')'),
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

fn part_1(input: String) -> i32 {
    let (directions, source_destination_pairs) = parse_inputs(&mut input.as_str());
    // dbg!(&directions, &source_destination_pairs);

    // walk!
    let mut curr_source = "AAA";
    let mut ans = 0;
    let mut directions_list = directions.chars().into_iter().collect::<Vec<char>>();

    let source_dest_map = source_destination_pairs
        .iter()
        .map(|(src, dest)| (*src, *dest))
        .collect::<BTreeMap<&str, (&str, &str)>>();

    while curr_source != "ZZZ" {
        if directions_list[ans] == 'L' {
            let curr_dest = source_dest_map.get(curr_source).unwrap().0;
            curr_source = curr_dest;
            ans += 1;
        } else {
            let curr_dest = source_dest_map.get(curr_source).unwrap().1;
            curr_source = curr_dest;
            ans += 1;
        }

        if directions_list.len() - 1 == ans && curr_source != "ZZZ" {
            // extend directions_list
            let mut new_directions_list = directions_list.clone();
            for c in directions_list {
                new_directions_list.push(c);
            }
            directions_list = new_directions_list;
        }
    }

    dbg!(ans);
    ans as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("./sample_input.txt");
        let output = part_1(input.to_string());
        assert_eq!(output, 6);
    }
}
