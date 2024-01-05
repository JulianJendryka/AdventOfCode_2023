use std::{collections::VecDeque, fs};

fn read_text(file_path: String) -> String {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    return contents;
}

#[derive(Clone, Debug)]
pub struct InitializationSequence {
    chars: VecDeque<char>,
}

impl InitializationSequence {
    pub fn create(line: String) -> InitializationSequence {
        InitializationSequence {
            chars: line.chars().into_iter().collect(),
        }
    }

    fn get_hash(&self) -> u128 {
        InitializationSequence::hash(0, self.chars.clone())
    }

    fn hash(current_value: u128, mut chars: VecDeque<char>) -> u128 {
        if chars.is_empty() {
            return current_value;
        }

        let current_char = chars.pop_front().unwrap();
        let current_char_acsi_num = (current_char as u8) as u128;
        let new_value = (current_value + current_char_acsi_num) * 17 % 256;

        InitializationSequence::hash(new_value, chars)
    }
}

fn parse_input(input: &String) -> Vec<InitializationSequence> {
    input
        .split(",")
        .into_iter()
        .map(|line| InitializationSequence::create(line.to_string()))
        .collect()
}

pub fn main() {
    println!("PART 1 ------------");

    let input = read_text("..\\Data\\input_1.txt".to_string());
    let initialization_sequence = parse_input(&input);

    let sum: u128 = initialization_sequence
        .into_iter()
        .map(|sequence| sequence.get_hash())
        .sum();
    println!("sum = {:?}", sum);
}
