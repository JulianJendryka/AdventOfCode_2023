use std::{collections::HashMap, fs};

fn read_text() -> String {
    let file_path = "..\\Data\\input_1.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    return contents;
}

fn extract_number_simple(line: &String) -> i32 {
    let mut all_numbers = String::new();

    for c in line.chars() {
        if c.is_numeric() {
            all_numbers.push(c);
        }
    }
    let mut start_and_end_numbers = String::new();
    start_and_end_numbers.push(all_numbers.chars().nth(0).unwrap());
    start_and_end_numbers.push(all_numbers.chars().nth(all_numbers.len() - 1).unwrap());

    return start_and_end_numbers.parse::<i32>().unwrap();
}

fn get_valid_numbers() -> HashMap<&'static str, char> {
    let valid_numbers = HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ]);
    return valid_numbers;
}

fn get_substring<'a>(line: &'a String, i: usize, word: &'a str) -> &'a str {
    let substring = &line[(i)..std::cmp::min(line.len(), i + word.len())];
    return substring;
}

fn extract_number_extra(line: &String) -> i32 {
    let valid_numbers = get_valid_numbers();

    let mut all_numbers = String::new();

    for i in 0..line.len() {
        let c = line.chars().nth(i).unwrap();
        if c.is_numeric() {
            all_numbers.push(c);
            continue;
        }

        for (word, value) in &valid_numbers {
            let substring = get_substring(line, i, word);

            let is_number = word.eq(&substring);
            if is_number {
                all_numbers.push(*value);
                continue;
            }
        }
    }

    let mut start_and_end_numbers = String::new();
    start_and_end_numbers.push(all_numbers.chars().nth(0).unwrap());
    start_and_end_numbers.push(all_numbers.chars().nth(all_numbers.len() - 1).unwrap());

    return start_and_end_numbers.parse::<i32>().unwrap();
}

fn part_one() {
    let input = read_text();

    let mut _sum = 0;
    for line in input.lines() {
        _sum += extract_number_simple(&line.to_string());
    }
}

fn part_two() {
    let input = read_text();

    let mut _sum = 0;
    for line in input.lines() {
        _sum += extract_number_extra(&line.to_string());
    }
}

fn main() {
    part_one();

    use std::time::Instant;
    let number_of_runs = 100;
    let mut now = Instant::now();
    for _ in 0..number_of_runs {
        part_one();
    }
    let elapsed = now.elapsed() / number_of_runs;
    println!("PART:1 ----- \nElapsed: {:.2?}", elapsed);

    now = Instant::now();
    for _ in 0..number_of_runs {
        part_two();
    }

    let elapsed = now.elapsed() / number_of_runs;
    println!("PART:2 ----- \nElapsed: {:.2?}", elapsed);
}
