use std::{
    collections::{HashMap, VecDeque},
    fs,
};

fn read_text(file_path: String) -> String {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    return contents;
}

#[derive(Debug)]
pub struct HotSpring {
    records: VecDeque<char>,
    nums: VecDeque<usize>,
}

impl Clone for HotSpring {
    fn clone(&self) -> HotSpring {
        HotSpring {
            records: self.records.clone(),
            nums: self.nums.clone(),
        }
    }
}

impl HotSpring {
    fn create_unfolded(left: String, right: String) -> HotSpring {
        let base = HotSpring::create(left, right);

        let mut records = base.records.clone();
        let mut nums = base.nums.clone();
        for _ in 0..(5 - 1) {
            records.push_back('?');
            records.append(&mut base.records.clone());

            nums.append(&mut base.nums.clone());
        }

        HotSpring {
            records: records,
            nums: nums,
        }
    }

    fn create(left: String, right: String) -> HotSpring {
        let records = left.chars().into_iter().collect();
        let nums = right
            .split(",")
            .into_iter()
            .map(|c| c.parse::<usize>().unwrap())
            .collect();

        HotSpring {
            records: records,
            nums: nums,
        }
    }

    fn shorten_record(&self) -> HotSpring {
        let mut new_shortened_records = self.records.clone();
        let _ = new_shortened_records.pop_front();
        HotSpring {
            records: new_shortened_records,
            nums: self.nums.clone(),
        }
    }

    fn shorten_record_and_nums(&self) -> HotSpring {
        let mut new_shortened_records = self.records.clone();
        let _ = new_shortened_records.pop_front();

        let mut new_shortened_nums = self.nums.clone();
        let _ = new_shortened_nums.pop_front();

        HotSpring {
            records: new_shortened_records,
            nums: new_shortened_nums,
        }
    }

    fn _print(&self) {
        for c in self.records.clone() {
            print!("{c}");
        }
        print!(" : ");
        for n in self.nums.clone() {
            print!("{n}");
        }
        print!("\n");
    }

    pub fn get_num_arrangements(&self, mappa: &mut HashMap<String, usize>) -> usize {
        let mut input = self.clone();
        input.records.push_back('.');

        get_num_arrangements_recursive(input, 0, mappa)
    }

    pub fn get_key(&self, size_of_current_group: usize) -> String {
        let records = self
            .records
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join("");

        let nums = self
            .nums
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join("_");

        let size = size_of_current_group.to_string();
        records + "_" + &nums + "_" + &size
    }
}

fn parse_input(input: &String) -> Vec<HotSpring> {
    input
        .split("\r\n")
        .into_iter()
        .map(|line| {
            let mut splitted = line.split(" ");
            let left = splitted.next().unwrap();
            let right = splitted.next().unwrap();
            HotSpring::create_unfolded(left.to_string(), right.to_string())
        })
        .collect()
}

fn get_num_arrangements_recursive(
    spring: HotSpring,
    size_of_current_group: usize,
    mappa: &mut HashMap<String, usize>,
) -> usize {
    let key = spring.get_key(size_of_current_group);
    if let Some(pre_computed_result) = mappa.get(&key) {
        return pre_computed_result.clone();
    }

    // println!("{key}");

    if let Some(is_valid_end) = handle_end(&spring, size_of_current_group) {
        return is_valid_end as usize;
    }

    let first_char = spring.records[0];
    let split_variations = match first_char {
        '?' => vec!['.', '#'],
        _ => vec![first_char],
    };

    let num_arrangements = split_variations
        .into_iter()
        .map(|v| handle_splits(v, &spring, size_of_current_group, mappa))
        .sum();

    mappa.insert(key, num_arrangements);
    num_arrangements
}

fn handle_splits(
    v: char,
    spring: &HotSpring,
    size_of_current_group: usize,
    mappa: &mut HashMap<String, usize>,
) -> usize {
    let mut num_arrangements = 0;

    let is_still_in_group = v == '#';
    match is_still_in_group {
        true => {
            num_arrangements += get_num_arrangements_recursive(
                spring.shorten_record(),
                size_of_current_group + 1,
                mappa,
            );
        }
        _ => {
            //ending a group or comming from "."
            let size_of_next_group = 0;
            let is_comming_from_group = size_of_current_group != 0;
            match is_comming_from_group {
                true => {
                    let does_group_size_match =
                        !spring.nums.is_empty() && size_of_current_group == spring.nums[0];

                    if does_group_size_match {
                        num_arrangements += get_num_arrangements_recursive(
                            spring.shorten_record_and_nums(),
                            size_of_next_group,
                            mappa,
                        );
                    }
                }
                false => {
                    num_arrangements += get_num_arrangements_recursive(
                        spring.shorten_record(),
                        size_of_next_group,
                        mappa,
                    );
                }
            };
        }
    };
    num_arrangements
}

fn handle_end(spring: &HotSpring, size_of_current_group: usize) -> Option<bool> {
    if spring.records.is_empty() {
        let is_group_empty = spring.nums.is_empty();
        let is_current_group_empty = size_of_current_group == 0;

        return Some(is_group_empty && is_current_group_empty);
    }
    None
}

pub fn main() {
    println!("PART 2 -----custom hashmap-------");

    let input = read_text("..\\Data\\input_1.txt".to_string());
    let springs = parse_input(&input);

    let mut map_to_rule_them_all: HashMap<String, usize> = HashMap::new();

    let all_sums: Vec<usize> = springs
        .into_iter()
        .map(|spring| spring.get_num_arrangements(&mut map_to_rule_them_all))
        .collect();

    // println!("all_sums {:?}", all_sums);
    let sum: usize = all_sums.into_iter().sum();
    println!("sum {:?}", sum);
}
