use std::fs;

fn main() {
    let file_path = "inputs/2022/day01_puzzle1";
    let contents = fs::read_to_string(file_path)
        .expect("No input yet");
    let mut elves: Vec<i64> = Vec::new();
    for elf in contents.split("\n\n") {
        elves.push(elf.lines()
                   .map(|l| l.parse::<i64>().unwrap())
                   .sum());
    }
    elves.sort_by(|a, b| b.cmp(a));
    println!("{}", elves[0] + elves[1] + elves[2]);
}
