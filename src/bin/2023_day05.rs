use std::fs;
use std::collections::HashMap;

// https://adventofcode.com/2023/day/5

fn parse_almanac(contents: String) -> u32 {
    //    let mut almanac = HashMap::new();
    let mut line_by_line = contents.split("\n");
    let first_line = line_by_line.next().unwrap();
    let (_title, seeds_string) = first_line.split_once(":").unwrap();
    let seeds: Vec<&str> = seeds_string.trim().split(' ').collect();
    println!("{:?}", seeds);
    println!("First line: {}", first_line);
    for line in line_by_line {
	if line.is_empty() {
	    continue;
	}
	println!("other line: {}", line);
    }
0
}

fn puzzle1(contents: String) -> u32 {
    let almanac = parse_almanac(contents);
    0
}

fn puzzle2(contents: String) -> u32 {
    for line in contents.split("\n") {
	println!("{}", line);
    }
    0
}


fn main() {
    let file_path = "inputs/2023/day05";
    let contents = fs::read_to_string(file_path)
        .expect("No input yet");
    let res1 = puzzle1(contents.clone());
    let res2 = puzzle2(contents.clone());
    println!("\nPuzzle 1: {}", res1);
    println!("Puzzle 2: {}", res2);
    
}

#[cfg(test)]
mod tests {
    use crate::*;

    const EXAMPLE_INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";

    #[test]
    fn puzzle1_example() {

        assert_eq!(puzzle1(EXAMPLE_INPUT.to_string()), 35);
    }

    #[test]
    fn puzzle2_example() {

        assert_eq!(puzzle2(EXAMPLE_INPUT.to_string()), 0);
    }

}
