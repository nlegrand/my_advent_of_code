use std::fs;
use std::fmt;

// https://adventofcode.com/2023/day/3

#[derive(Debug, Clone, Copy)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn add(&self, vector: Position) -> Position {
	Position { x: self.x + vector.x,
		   y: self.y + vector.y }
    }
    fn equals(&self, position: Position) -> bool {
	self.x == position.x && self.y == position.y
    }
    fn is_adjacent(&self, position:Position) -> bool {
	let adjacent_vectors = [
	    Position { x: -1, y: 0 },
	    Position { x: -1, y: 1 },
	    Position { x: -1, y: -1 },
	    Position { x: 0, y: 1 },
	    Position { x: 0, y: -1 },
	    Position { x: 1, y: 0 },
	    Position { x: 1, y: -1 },
	    Position { x: 1, y: 1 },
	];
	for vector in adjacent_vectors {
	    if self.add(vector).equals(position) {
		return true;
	    }
	}
	return false;
    }
}

impl fmt::Display for Position {
	/// Print feat dice
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	write!(f, "x: {}, y: {}", self.x, self.y)
    }
}


fn puzzle1(contents: String) -> u32 {
    let mut line_number = 0;
    let mut column_number = 0;

    let mut symbols_pos = vec![];
    // parse for symbols
    for c in contents.chars() {
	if c.is_ascii_punctuation() && c != '.' {
	    println!("Symbol {} detected at pos:  ({}, {})", c, column_number, line_number);
	    symbols_pos.push(Position {x: column_number, y: line_number})
	}
	column_number += 1;
	if c == '\n' {
	    line_number += 1;
	    column_number = 0;
	}
    }
    println!("{:?}", symbols_pos);

    //parse for numbers and check if symbol is adjacent
    line_number = 0;
    column_number = 0;
    let mut digits = vec![];
    let mut part_numbers: Vec<u32> = vec![]; //our results
    let mut is_part_number = false;
    for c in contents.chars() {
	if c.is_ascii_digit() {
	    let cur_point = Position { x: column_number, y: line_number };
	    for symbol in &symbols_pos {
		if cur_point.is_adjacent(*symbol) {
		    is_part_number = true;
		    println!("Char {} and Point {} are part of a part number adjacent to symbol {}", c, cur_point, symbol);
		}
	    }
	    digits.push(c.to_string().parse::<u32>().unwrap());
	}
	else {
	    if !digits.is_empty() {
		if is_part_number {
		    let mut l = digits.len();
		    let mut part_number = 0;
		    for i in &digits {
			part_number += 10_u32.pow((l - 1).try_into().unwrap()) * i;
			l -= 1;
		    }
		    println!("Partnumber: {}", part_number);
		    part_numbers.push(part_number);
		}
		digits = vec![];
		is_part_number = false;		
	    }
	    if c == '\n' {
		line_number += 1;
		column_number = 0;
		digits = vec![];
		is_part_number = false;
		continue;
	    }
	}
	column_number += 1;
    }
    let res = part_numbers.iter().sum();
    println!("Res vec puzzle 1: {:?}", part_numbers);
    println!("Res puzzle 1: {}", res);
    res
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

    #[test]
    fn position_equality() {
	let pos1 = Position { x: 5, y: 7 };
	let pos2 = Position { x: 5, y: 7 };
	let pos3 = Position { x: 2, y: 3 };
	assert!(pos1.equals(pos2));
	assert!(!pos1.equals(pos3));
    }

    #[test]
    fn is_adjacent() {
	let pos1 = Position { x: 5, y: 7 };
	let pos2 = Position { x: 5, y: 8 };
	let pos3 = Position { x: 6, y: 8 };
	let pos4 = Position { x: 6, y: 7 };
	let pos5 = Position { x: 6, y: 6 };
	let pos6 = Position { x: 5, y: 6 };
	let pos7 = Position { x: 4, y: 6 };
	let pos8 = Position { x: 4, y: 7 };
	let pos9 = Position { x: 4, y: 8 };
	let pos10 = Position { x: 5, y: 9 };
	assert!(pos1.is_adjacent(pos2));
	assert!(pos1.is_adjacent(pos3));
	assert!(pos1.is_adjacent(pos4));
	assert!(pos1.is_adjacent(pos5));
	assert!(pos1.is_adjacent(pos6));
	assert!(pos1.is_adjacent(pos7));
	assert!(pos1.is_adjacent(pos8));
	assert!(pos1.is_adjacent(pos9));
	assert!(!pos1.is_adjacent(pos10));
    }

    const EXAMPLE_INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn puzzle1_example() {
        assert_eq!(puzzle1(EXAMPLE_INPUT.to_string()), 4361);
    }

    #[test]
    fn puzzle2_example() {

        assert_eq!(puzzle2(EXAMPLE_INPUT.to_string()), 0);
    }

}
