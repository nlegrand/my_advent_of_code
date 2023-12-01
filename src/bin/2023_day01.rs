use std::fs;

// https://adventofcode.com/2023/day/1

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
    0
}

fn main() {
    let file_path = "inputs/2023/day01";
    let contents = fs::read_to_string(file_path)
        .expect("No input yet");
    let res1 = puzzle1(contents);
    println!("Puzzle 1: {}", res1);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn puzzle1_example() {
        
        assert_eq!(puzzle1(EXAMPLE_INPUT_1.to_string()), 142);
    }

    #[test]
    fn puzzle2_example() {
        
        assert_eq!(puzzle2(EXAMPLE_INPUT_2.to_string()), 281);
    }

}
