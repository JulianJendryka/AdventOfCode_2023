use cached::proc_macro::cached;
use std::{collections::VecDeque, fs};

fn read_text(file_path: String) -> String {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    return contents;
}

fn create(left: String, right: String) -> (VecDeque<char>, VecDeque<usize>) {
    let records: VecDeque<char> = left.chars().into_iter().collect();
    let nums: VecDeque<usize> = right
        .split(",")
        .into_iter()
        .map(|c| c.parse::<usize>().unwrap())
        .collect();

    (records, nums)
}

fn create_unfolded(left: String, right: String) -> (VecDeque<char>, VecDeque<usize>) {
    let (base_records, base_nums) = create(left, right);

    let mut records = base_records.clone();
    let mut nums = base_nums.clone();
    for _ in 0..(5 - 1) {
        records.push_back('?');
        records.append(&mut base_records.clone());

        nums.append(&mut base_nums.clone());
    }
    (records, nums)
}

fn shorten<T>(mut record: VecDeque<T>) -> VecDeque<T> {
    let _ = record.pop_front();
    record
}

pub fn get_num_arrangements(x: &(VecDeque<char>, VecDeque<usize>)) -> usize {
    let (base_records, base_nums) = x;

    let mut base_records2 = base_records.clone();
    base_records2.push_back('.');
    get_num_arrangements_recursive(base_records2, base_nums.clone(), 0)
}

#[cached]
fn get_num_arrangements_recursive(
    record: VecDeque<char>,
    nums: VecDeque<usize>,
    size_of_current_group: usize,
) -> usize {
    if let Some(is_valid_end) = handle_end(&record, &nums, size_of_current_group) {
        return is_valid_end as usize;
    }

    let first_char = record[0];
    let split_variations = match first_char {
        '?' => vec!['.', '#'],
        _ => vec![first_char],
    };

    let num_arrangements = split_variations
        .into_iter()
        .map(|v| handle_splits(v, &record, &nums, size_of_current_group))
        .sum();

    num_arrangements
}

fn handle_splits(
    v: char,
    record: &VecDeque<char>,
    nums: &VecDeque<usize>,
    size_of_current_group: usize,
) -> usize {
    let mut num_arrangements = 0;

    let is_still_in_group = v == '#';
    match is_still_in_group {
        true => {
            num_arrangements += get_num_arrangements_recursive(
                shorten(record.clone()),
                nums.clone(),
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
                        !nums.is_empty() && size_of_current_group == nums[0];

                    if does_group_size_match {
                        num_arrangements += get_num_arrangements_recursive(
                            shorten(record.clone()),
                            shorten(nums.clone()),
                            size_of_next_group,
                        );
                    }
                }
                false => {
                    num_arrangements += get_num_arrangements_recursive(
                        shorten(record.clone()),
                        nums.clone(),
                        size_of_next_group,
                    );
                }
            };
        }
    };
    num_arrangements
}

fn handle_end(
    record: &VecDeque<char>,
    nums: &VecDeque<usize>,
    size_of_current_group: usize,
) -> Option<bool> {
    if record.is_empty() {
        let is_group_empty = nums.is_empty();
        let is_current_group_empty = size_of_current_group == 0;

        return Some(is_group_empty && is_current_group_empty);
    }
    None
}

fn parse_input(input: &String) -> Vec<(VecDeque<char>, VecDeque<usize>)> {
    input
        .split("\r\n")
        .into_iter()
        .map(|line| {
            let mut splitted = line.split(" ");
            let left = splitted.next().unwrap();
            let right = splitted.next().unwrap();
            create_unfolded(left.to_string(), right.to_string())
        })
        .collect()
}

pub fn main() {
    println!("PART 2 -----cached = 0.46.1 -------");

    let input = read_text("..\\Data\\input_1.txt".to_string());
    let springs = parse_input(&input);

    let all_sums: Vec<usize> = springs
        .into_iter()
        .map(|spring| get_num_arrangements(&spring))
        .collect();

    let sum: usize = all_sums.into_iter().sum();
    println!("sum {:?}", sum);
}
