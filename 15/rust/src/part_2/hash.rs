use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub struct HashCreator {}

impl HashCreator {
    pub fn get_hash(line: &String) -> u128 {
        let chars: VecDeque<char> = line.chars().into_iter().collect();
        HashCreator::hash(0, chars.clone())
    }

    fn hash(current_value: u128, mut chars: VecDeque<char>) -> u128 {
        if chars.is_empty() {
            return current_value;
        }

        let current_char = chars.pop_front().unwrap();
        let current_char_acsi_num = (current_char as u8) as u128;
        let new_value = (current_value + current_char_acsi_num) * 17 % 256;

        HashCreator::hash(new_value, chars)
    }
}
