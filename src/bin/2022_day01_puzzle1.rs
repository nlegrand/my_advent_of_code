use std::fs;

fn main() {
    let file_path = "inputs/2022/day01_puzzle1";
    let contents = fs::read_to_string(file_path)
        .expect("No input yet");
    let mut current_elf = 1 ; // $ grep -e '^$' inputs/day_01_input_1 |wc -l
                      // 254
    let mut current_calories = 0;
    let mut most_calories = 0;
    let mut most_calories_carrier = 0;
    for line in contents.lines() {
        if line.is_empty() {
            println!("{}, {}", current_elf, current_calories);            
            if current_calories > most_calories {
                most_calories = current_calories;
                most_calories_carrier = current_elf;
            }
            current_calories = 0;
            current_elf += 1;
        }
        else {
            current_calories += line.parse::<usize>().unwrap();
        }
    }
    if (current_calories > 0) {
	println!("{}, {}", current_elf, current_calories);            
            if current_calories > most_calories {
                most_calories = current_calories;
                most_calories_carrier = current_elf;
            }
    }
    println!("Elf {} carried the most calories: {}", most_calories_carrier, most_calories) ;
}
