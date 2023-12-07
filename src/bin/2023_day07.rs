use std::fs;

// https://adventofcode.com/2023/day/6

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

fn label_to_card (label: &str) -> Option<Card> {
    match label {
        "A" => Some(Card::Ace),
        "K" => Some(Card::King),
        "Q" => Some(Card::Queen),
        "J" => Some(Card::Jack),
        "T" => Some(Card::Ten),
        "9" => Some(Card::Nine),
        "8" => Some(Card::Eight),
        "7" => Some(Card::Seven),
        "6" => Some(Card::Six),
        "5" => Some(Card::Five),
        "4" => Some(Card::Four),
        "3" => Some(Card::Three),
        "2" => Some(Card::Two),
        _ => None,
    }
}

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
    let file_path = "inputs/2023/day06";
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

    const EXAMPLE_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn cards_comparison() {
        assert!(Card::Ace > Card::King);
        assert!(Card::King > Card::Queen);
        assert!(Card::Queen > Card::Jack);
        assert!(Card::Jack > Card::Ten);
        assert!(Card::Ten > Card::Nine);
        assert!(Card::Nine > Card::Eight);
        assert!(Card::Seven > Card::Six);
        assert!(Card::Five > Card::Four);
        assert!(Card::Four > Card::Three);
        assert!(Card::Two < Card::Three);
    }

    #[test]
    fn card_construction() {
        let labels = vec!["A", "K", "Q", "J", "T", "9", "8", "7", "6", "5", "4", "3", "2"];
        for card in labels.iter().map(|l| label_to_card(l)) {
            println!("{:?}", card);
            assert!(match card {
                Some(Card::Ace) => true,
                Some(Card::King) => true,
                Some(Card::Queen) => true,
                Some(Card::Jack) => true,
                Some(Card::Ten) => true,
                Some(Card::Nine) => true,
                Some(Card::Eight) => true,
                Some(Card::Seven) => true,
                Some(Card::Six) => true,
                Some(Card::Five) => true,
                Some(Card::Four) => true,
                Some(Card::Three) => true,
                Some(Card::Two) => true,
                _ => false,
            });
        }
    }
    
    #[test]
    fn puzzle1_example() {

        assert_eq!(puzzle1(EXAMPLE_INPUT.to_string()), 6440);
    }


    #[test]
    fn puzzle2_example() {

        assert_eq!(puzzle2(EXAMPLE_INPUT.to_string()), 0);
    }

}
