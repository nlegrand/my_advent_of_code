use std::fs;

// https://adventofcode.com/2023/day/1

fn puzzle1(contents: String) -> u32 {
    let mut calibration_values = vec![];
    for line in contents.split("\n") {
        let mut first = 0;
        let mut last = 0;
        for c in line.chars() {
            if c.is_ascii_digit() {
                if first == 0 {
                    first = c.to_digit(10).unwrap();
                }
                last = c.to_digit(10).unwrap();
            }
        }
        calibration_values.push(first * 10 + last);
    }
    calibration_values.iter().sum()
}

fn puzzle2(contents: String) -> u32 {
    let pat = ["one", "two","three","four","five","six","seven","eight","nine","1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let mut calibration_values = vec![];
    for line in contents.split("\n") {
        if line.is_empty() {
            continue;
        }
        let mut digit_v: Vec<_> = vec![];
        for pattern in pat {
            digit_v.append(&mut line.match_indices(pattern).collect::<Vec<_>>());
        }
        digit_v.sort_by(|a, b| a.0.cmp(&b.0));
        let first =  digit_v.iter().next();
        let last = digit_v.iter().last();
        calibration_values.push(str_to_int(first.unwrap().1) * 10 + str_to_int(last.unwrap().1));

    }
    calibration_values.iter().sum()
}

fn str_to_int(str_digit: &str) -> u32 {
    match str_digit {
        "one" => 1,
        "1" => 1,
        "two" => 2,
        "2" => 2,
        "three" => 3,
        "3" => 3,
        "four" => 4,
        "4" => 4,
        "five" => 5,
        "5" => 5,
        "six" => 6,
        "6" => 6,
        "seven" => 7,
        "7" => 7,
        "eight" => 8,
        "8" => 8,
        "nine" => 9,
        "9" => 9,
        _ => panic!("Not a digit!"),
    }
}
    

fn main() {
    let file_path = "inputs/2023/day01";
    let contents = fs::read_to_string(file_path)
        .expect("No input yet");
    let res1 = puzzle1(contents.clone());
    println!("Puzzle 1: {}", res1);
    let res2 = puzzle2(contents.clone());
    println!("Puzzle 2: {}", res2);
    
}

#[cfg(test)]
mod tests {
    use crate::*;

    const EXAMPLE_INPUT_1: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    const EXAMPLE_INPUT_2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";


    #[test]
    fn puzzle1_example() {

        assert_eq!(puzzle1(EXAMPLE_INPUT_1.to_string()), 142);
    }

    #[test]
    fn puzzle2_example() {

        assert_eq!(puzzle2(EXAMPLE_INPUT_2.to_string()), 281);
    }

}
