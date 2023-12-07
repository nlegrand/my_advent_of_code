use std::fs;
use std::collections::HashMap;

// https://adventofcode.com/2023/day/7

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Copy, Clone)]
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

fn label_to_card (label: &char) -> Option<Card> {
    match label {
        'A' => Some(Card::Ace),
        'K' => Some(Card::King),
        'Q' => Some(Card::Queen),
        'J' => Some(Card::Jack),
        'T' => Some(Card::Ten),
        '9' => Some(Card::Nine),
        '8' => Some(Card::Eight),
        '7' => Some(Card::Seven),
        '6' => Some(Card::Six),
        '5' => Some(Card::Five),
        '4' => Some(Card::Four),
        '3' => Some(Card::Three),
        '2' => Some(Card::Two),
        _ => None,
    }
}

fn labels_to_hand (labels: &str) -> Option<Hand> {
    if labels.len() != 5 {
        eprintln!("Invalid hand. A valid hand has five labels {}", labels);
        return None;
    }
    let mut hand_v = vec![];
    for label in labels.chars() {
        hand_v.push(label_to_card(&label).unwrap());
    }
    hand_v.sort_by(|a, b| b.cmp(a));
    let mut count_cards = HashMap::new();
    for card in &hand_v {
        let count = count_cards.entry(card).or_insert(0);
        *count += 1;
    }
    let mut has_pair = None;
    for (key, value) in &count_cards {
        if *value == 2 {
            has_pair = Some(key);
        }
    }
    let mut top_value = 0;
    let mut top_card = Card::Two;
    for (key, value) in &count_cards {
        if value > &top_value {
            top_value = *value;
            top_card = **key;
        }
        if top_value == 1 && &top_card < key {
            top_card = **key;
        }
    }
    if top_value == 3 && has_pair != None {
        return Some(Hand::FullHouse(top_card, **has_pair.unwrap()));
    }
    if top_value == 2 && has_pair != None {
	if top_card < **has_pair.unwrap() {
            return Some(Hand::TwoOfAKind(**has_pair.unwrap()));
	}
	else {
	    return Some(Hand::TwoOfAKind(top_card));
	}
    }
    else {
        match top_value {
            5 => return Some(Hand::FiveOfAKind(top_card)),
            4 => return Some(Hand::FourOfAKind(top_card)),
            3 => return Some(Hand::ThreeOfAKind(top_card)),
            2 => return Some(Hand::TwoOfAKind(top_card)),
            1 => return Some(Hand::HighCard(top_card)),
            _ => return None,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Hand {
    HighCard(Card),
    TwoOfAKind(Card),
    ThreeOfAKind(Card),
    FullHouse(Card, Card),
    FourOfAKind(Card),
    FiveOfAKind(Card),
}

fn puzzle1(contents: String) -> u32 {
    let mut hands_and_bids = vec![];
    for line in contents.split("\n") {
        if line.is_empty() {
            continue;
        }
	//println!("{}", line);
        let (hand, bid) = line.split_once(' ').unwrap();
        hands_and_bids.push((labels_to_hand(hand).unwrap(), bid, hand));
        
    }
    hands_and_bids.sort_by(|a, b| a.0.cmp(&b.0));
    let mut rank = 1;
    let mut result = 0;
    for (hand, bid, hand_str) in hands_and_bids {
	result += rank * bid.parse::<u32>().unwrap();
	println!("{} -> {:?}, Bid: {}, Rank: {}, winnings: {}", hand_str, hand, bid, rank, result);
	rank += 1;
    }
    //println!("{:?}", hands_and_bids);
    println!("{}", result);
    result
}

fn puzzle2(_contents: String) -> u32 {
    // for line in contents.split("\n") {
    // 	//println!("{}", line);
    // }
    0
}


fn main() {
    let file_path = "inputs/2023/day07";
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
    fn hand_comparison() {
        assert!(Hand::FiveOfAKind(Card::Two) > Hand::FourOfAKind(Card::Ace));
        assert!(Hand::FourOfAKind(Card::Three) > Hand::FullHouse(Card::King, Card::Queen));
        assert!(Hand::FullHouse(Card::Four, Card::Jack) > Hand::ThreeOfAKind(Card::Five));
        assert!(Hand::ThreeOfAKind(Card::Six) > Hand::TwoOfAKind(Card::Queen));
        assert!(Hand::TwoOfAKind(Card::Seven) > Hand::HighCard(Card::Ten));
        assert!(Hand::FiveOfAKind(Card::Three) < Hand::FiveOfAKind(Card::Four));
        assert!(Hand::FullHouse(Card::Three, Card::Ace) < Hand::FullHouse(Card::Four, Card::Two));
    }

    #[test]
    fn card_construction() {
        let labels = vec!['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];
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
