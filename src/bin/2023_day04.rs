use std::fs;
use std::collections::HashMap; // for puzzle 2

// https://adventofcode.com/2023/day/4

fn crunch_points(matches: Vec<&str>) -> u32 {
    let mut res = 0;
    if matches.len() == 1 {
	res = 1;
    }
    else if matches.len() > 1 {
	res = 1;
	for _p in 1..matches.len() {
            res *= 2;
	}
    }
    res
}

fn puzzle1(contents: String) -> u32 {
    let mut res = vec![];
    for line in contents.split("\n") {
	if line.is_empty() {
	    continue;
	}
	let card  = line.split_once(':').unwrap();
	let (winning_numbers_raw, numbers_you_have_raw) = card.1.split_once('|').unwrap();
	let winning_numbers: Vec<_> = winning_numbers_raw.trim().split(' ')
	    .filter(|s| ! s.is_empty() ).collect();
	let numbers_you_have = numbers_you_have_raw.trim().split(' ')
	    .filter(|s| ! s.is_empty() );
	
	let matches: Vec<_> = numbers_you_have.filter(|s| winning_numbers.contains(s)).collect();
	let crunched = crunch_points(matches.clone()); 
	println!("{:?} and the result is {}", matches, crunched);
	res.push(crunched);
    }
    let res_sum = res.iter().sum();
    println!("Res sum: {}", res_sum);
    res_sum
}

fn puzzle2(contents: String) -> u32 {
    let mut scratchcards = HashMap::new();
    for line in contents.split("\n") {
	if line.is_empty() {
	    continue;
	}
	let (card, content)  = line.split_once(':').unwrap();
	let (_card, id_str) = card.split_once(' ').unwrap();
	println!("id_str: {}", id_str.trim());
	let id = id_str.trim().to_string().parse::<usize>().unwrap();
	scratchcards.entry(id).or_insert(1);
	let (winning_numbers_raw, numbers_you_have_raw) = content.split_once('|').unwrap();
	let winning_numbers: Vec<_> = winning_numbers_raw.trim().split(' ')
	    .filter(|s| ! s.is_empty() ).collect();
	let numbers_you_have = numbers_you_have_raw.trim().split(' ')
	    .filter(|s| ! s.is_empty() );
	
	let matches: Vec<_> = numbers_you_have.filter(|s| winning_numbers.contains(s)).collect();
	for y in 0..scratchcards.get(&id).copied().unwrap_or(1) {
	    println!("y: {}", y);
	    for i in 1..(matches.len() + 1) {
		println!("id: {}, i: {}, i + id = {}", id, i, i + id);
		let count = scratchcards.entry(i + id).or_insert(1);
		*count += 1;
	    }
	}
    }
    let mut res = 0;
    for (key, val) in scratchcards.iter() {
	println!("key: {key} val: {val}");
	res += val;
    }
    u32::try_from(res).unwrap()
}


fn main() {
    let file_path = "inputs/2023/day04";
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

    const EXAMPLE_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn puzzle1_example() {

        assert_eq!(puzzle1(EXAMPLE_INPUT.to_string()), 13);
    }

    #[test]
    fn puzzle2_example() {

        assert_eq!(puzzle2(EXAMPLE_INPUT.to_string()), 30);
    }

}
