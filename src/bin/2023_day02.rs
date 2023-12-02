use std::fs;

// https://adventofcode.com/2023/day/2



fn puzzle1(contents: String) -> u32 {
    let mut ids = vec![0u32];
    for line in contents.split("\n") {
	let game = line.split_once(':').unwrap(); //let game = (ngame, subsets)
	let id = game.0.split_once(' ').unwrap(); //let id = ("Game", "1")
	let mut impossible = false;
	println!("{} {}", id.0, id.1);
	for subset in game.1.split(';') {
	    for cube in subset.split(',') {
		let res = cube.trim().split_once(' ').unwrap(); // let res = ("1", "red")
		println!("  {:?}", res);
		impossible = match res {
		    (n, "red") if n.parse::<u32>().unwrap() >= 12 => true,
		    (n, "green") if n.parse::<u32>().unwrap() >= 13 => true,
		    (n, "blue") if n.parse::<u32>().unwrap() >= 14 => true,
		     _ => impossible,
		}
	    }
	}
	if impossible {
	    println!("Impossible: {}", game.0);
	}
	else {
	    ids.push(id.1.parse::<u32>().unwrap());
	}
    }
    let ret = ids.iter().sum();
    println!("Res: {}", ret);
    ret
}

fn puzzle2(contents: String) -> u32 {
    for line in contents.split("\n") {
    }
    0
}


fn main() {
    let file_path = "inputs/2023/day02";
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

    const EXAMPLE_INPUT_1: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    const EXAMPLE_INPUT_2: &str = "";


    #[test]
    fn puzzle1_example() {

        assert_eq!(puzzle1(EXAMPLE_INPUT_1.to_string()), 8);
    }

    #[test]
    fn puzzle2_example() {

        assert_eq!(puzzle2(EXAMPLE_INPUT_2.to_string()), 0);
    }

}
