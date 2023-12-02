fn main() {
    let input = include_str!("./input.txt");
    let output = part_1(input);
    dbg!(output);
}

#[allow(unused_variables)]
fn part_1(input: &str) -> i32 {
    println!("Raw input: {:?}", input);
    let data: Vec<&str> = input.split("\n").collect();
    let mut final_ans: i32 = 0;
    let mut temp_num: i32 = 0;
    for i in data {
        println!("{:?}", i);
        for j in i.chars() {
            println!("{:?}", j);
            if j.is_digit(10) {
                temp_num = j.to_digit(10).unwrap() as i32;
                break;
            }
        }
        for j in i.chars().rev() {
            println!("{:?}", j);
            if j.is_digit(10) {
                temp_num = temp_num * 10 + j.to_digit(10).unwrap() as i32;
                break;
            }
        }
        println!("{:?}", temp_num);
        final_ans += temp_num;
        temp_num = 0;
    }
    println!("{:?}", final_ans);
    final_ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";
        let output = part_1(input);
        assert_eq!(output, 142);
    }
}
