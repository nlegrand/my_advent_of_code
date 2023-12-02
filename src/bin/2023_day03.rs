use std::fs;

// https://adventofcode.com/2023/day/3


fn puzzle1(contents: String) -> u32 {
    for line in contents.split("\n") {
	println!("{}", line);
    }
    0
}

fn puzzle2(contents: String) -> u32 {
    for line in contents.split("\n") {
	println!("{}", line);
    }
    0
}


fn main() {
    let file_path = "inputs/2023/day03";
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

    const EXAMPLE_INPUT: &str = "";

    #[test]
    fn puzzle1_example() {

        assert_eq!(puzzle1(EXAMPLE_INPUT.to_string()), 0);
    }

    #[test]
    fn puzzle2_example() {

        assert_eq!(puzzle2(EXAMPLE_INPUT.to_string()), 0);
    }

}
