use itertools::Itertools;
use nom::{
    bytes::complete::tag, character::complete::space0, character::complete::space1,
    sequence::preceded, IResult,
};

fn main() {
    let input = include_str!("./input.txt");
    let trimmed_input = trim_extra_spaces(input);
    println!("{:?}", trimmed_input);
    let output = part_1(trimmed_input.as_str());
    dbg!(&output.unwrap().1);
}

fn trim_extra_spaces(input: &str) -> String {
    input.trim().split(' ').filter(|s| !s.is_empty()).join(" ")
}

fn part_1(input: &str) -> IResult<&str, i32> {
    let mut ans = 0;

    for line in input.lines() {
        let (line_details, _) = preceded(tag("Card "), space0)(line)?;
        dbg!(line_details);

        let remaining = line_details.split(": ").collect::<Vec<&str>>()[1];
        dbg!(remaining);
        let cards = remaining.split(" | ").collect::<Vec<&str>>();
        let winning = cards[0]
            .trim()
            .split(' ')
            .filter(|s| !s.is_empty())
            .join(" ");
        let available = cards[1]
            .trim()
            .split(' ')
            .filter(|s| !s.is_empty())
            .join(" ");
        dbg!(&winning, &available);
        let winning = winning
            .split(" ")
            .into_iter()
            .map(|digit| digit.trim().parse::<i32>().unwrap())
            .collect::<Vec<i32>>();
        let available = available
            .split(" ")
            .into_iter()
            .map(|digit| digit.trim().parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        let mut temp_ans = 0;

        for i in winning {
            if available.contains(&i) {
                dbg!(i, temp_ans);
                if temp_ans == 0 {
                    temp_ans = 1;
                } else {
                    temp_ans = temp_ans * 2;
                }
            }
        }
        dbg!(ans, temp_ans);

        ans += temp_ans;
    }

    Ok((input, ans))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("./input.txt");
        let output = part_1(input).unwrap();
        assert_eq!(output.1, 13);
    }
}
