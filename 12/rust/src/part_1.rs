use std::{collections::VecDeque, fs};

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
    pub fn create(left: String, right: String) -> HotSpring {
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

    fn shorten_record(spring: &HotSpring) -> HotSpring {
        let mut new_shortened_records = spring.records.clone();
        let _ = new_shortened_records.pop_front();
        HotSpring {
            records: new_shortened_records,
            nums: spring.nums.clone(),
        }
    }

    fn shorten_record_and_nums(spring: &HotSpring) -> HotSpring {
        let mut new_shortened_records = spring.records.clone();
        let _ = new_shortened_records.pop_front();

        let mut new_shortened_nums = spring.nums.clone();
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

    pub fn get_num_arrangements(&self) -> usize {
        let mut x = self.clone();
        x.records.push_back('.');
        HotSpring::get_num_arrangements_recursive(x, 0)
    }

    fn get_num_arrangements_recursive(spring: HotSpring, size_of_current_group: usize) -> usize {
        if let Some(is_valid_end) = HotSpring::handle_end(&spring, size_of_current_group) {
            return is_valid_end as usize;
        }

        // spring.print();

        let first_char = spring.records[0];
        let split_variations = match first_char {
            '?' => vec!['.', '#'],
            _ => vec![first_char],
        };

        let mut num_arrangements = 0;
        for v in split_variations {
            num_arrangements += HotSpring::handle_splits(v, &spring, size_of_current_group);
        }
        num_arrangements
    }

    fn handle_splits(v: char, spring: &HotSpring, size_of_current_group: usize) -> usize {
        let mut num_arrangements = 0;

        let is_still_in_group = v == '#';
        match is_still_in_group {
            true => {
                num_arrangements += HotSpring::get_num_arrangements_recursive(
                    HotSpring::shorten_record(spring),
                    size_of_current_group + 1,
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
                            num_arrangements += HotSpring::get_num_arrangements_recursive(
                                HotSpring::shorten_record_and_nums(spring),
                                size_of_next_group,
                            );
                        }
                    }
                    false => {
                        num_arrangements += HotSpring::get_num_arrangements_recursive(
                            HotSpring::shorten_record(spring),
                            size_of_next_group,
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
}

fn parse_input(input: &String) -> Vec<HotSpring> {
    input
        .split("\r\n")
        .into_iter()
        .map(|line| {
            let mut splitted = line.split(" ");
            let left = splitted.next().unwrap();
            let right = splitted.next().unwrap();
            HotSpring::create(left.to_string(), right.to_string())
        })
        .collect()
}
pub fn main() {
    println!("PART 1 ------------");

    let input = read_text("..\\Data\\input_1.txt".to_string());
    let springs = parse_input(&input);

    // println!("springs {:?}", springs);

    springs[0].get_num_arrangements();

    let all_sums: Vec<usize> = springs
        .into_iter()
        .map(|spring| spring.get_num_arrangements())
        .collect();

    // println!("all_sums {:?}", all_sums);
    let sum: usize = all_sums.into_iter().sum();
    println!("sum {:?}", sum);
}
