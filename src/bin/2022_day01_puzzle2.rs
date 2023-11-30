use std::fs;

// https://adventofcode.com/2022/day/1

fn puzzle(contents: String) -> i64 {
    let mut elves: Vec<i64> = Vec::new();
    for elf in contents.split("\n\n") {
        elves.push(elf.lines()
                   .map(|l| l.parse::<i64>().unwrap())
                   .sum());
    }
    elves.sort_by(|a, b| b.cmp(a));
    elves[0] + elves[1] + elves[2]
}

fn main() {
    let file_path = "inputs/2022/day01_puzzle1";
    let contents = fs::read_to_string(file_path)
        .expect("No input yet");
    let answer = puzzle(contents);
    println!("{}", answer);
}

#[cfg(test)]
mod tests {
    use crate::puzzle;
    #[test]
    fn puzzle_demo() {
        let input = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000".to_string();
        assert_eq!(puzzle(input), 45000);
    }
}
