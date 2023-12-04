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
    let mut symbols_pos = vec![];
    for line in contents.split("\n") {
	if line.is_empty() {
	    continue;
	}
	let mut column_number = 0;
	for c in line.chars() {
	    if c.is_ascii_punctuation() && c != '.' {
		println!("Symbol detected at pos:  ({}, {})", column_number, line_number);
		symbols_pos.push(Position {x: column_number, y: line_number})
	    }
	    column_number += 1;
	}
	line_number += 1;
    }
    println!("{:?}", symbols_pos);
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

    #[test]
    fn position_equality() {
	let pos1 = Position { x: 5, y: 7 };
	let pos2 = Position { x: 5, y: 7 };
	let pos3 = Position { x: 2, y: 3 };
	assert_eq!(pos1.equals(pos2), true);
	assert_eq!(pos1.equals(pos3), false);
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
	assert_eq!(pos1.is_adjacent(pos2), true);
	assert_eq!(pos1.is_adjacent(pos3), true);
	assert_eq!(pos1.is_adjacent(pos4), true);
	assert_eq!(pos1.is_adjacent(pos5), true);
	assert_eq!(pos1.is_adjacent(pos6), true);
	assert_eq!(pos1.is_adjacent(pos7), true);
	assert_eq!(pos1.is_adjacent(pos8), true);
	assert_eq!(pos1.is_adjacent(pos9), true);
	assert_eq!(pos1.is_adjacent(pos10), false);
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
