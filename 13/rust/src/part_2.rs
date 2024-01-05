use std::{collections::HashMap, fs};

fn read_text(file_path: String) -> String {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    return contents;
}

fn parse_input(input: &String) -> Vec<MirrorField> {
    input
        .split("\r\n\r\n")
        .into_iter()
        .map(|block| MirrorField::create(block))
        .collect()
}

pub struct MirrorField {
    _lines: Vec<String>,
    _lines_transposed: Vec<String>,

    row_hashs: Vec<usize>,
    col_hashs: Vec<usize>,
}
impl MirrorField {
    pub fn create(block: &str) -> MirrorField {
        let lines: Vec<String> = block
            .split("\r\n")
            .into_iter()
            .map(|line| line.to_string())
            .collect();

        let transposed = MirrorField::transpose(&lines);
        MirrorField {
            _lines: lines.clone(),
            _lines_transposed: transposed.clone(),
            row_hashs: MirrorField::get_line_map(&lines),
            col_hashs: MirrorField::get_line_map(&transposed),
        }
    }

    fn get_line_map(lines: &Vec<String>) -> Vec<usize> {
        let line_map: HashMap<String, usize> = lines
            .iter()
            .enumerate()
            .map(|(index, line)| (line.clone(), index))
            .collect();

        lines.iter().map(|line| line_map[line]).collect()
    }

    fn transpose(line_input: &Vec<String>) -> Vec<String> {
        let lines: Vec<Vec<char>> = line_input
            .into_iter()
            .map(|line| line.chars().collect())
            .collect();

        let rows = lines.len();
        let cols = lines[0].len();

        (0..cols)
            .map(|col| (0..rows).map(|row| lines[row][col]).collect())
            .collect()
    }

    pub fn _print(&self) {
        for l in self._lines.iter() {
            println!("{:?}", l);
        }
        println!("self.row_hashs : {:?}", self.row_hashs);
        println!("self.col_hashs : {:?}", self.col_hashs);
    }
}

fn does_contain_each_other_possibly(
    left: &Vec<usize>,
    right: &Vec<usize>,
) -> Option<Vec<(usize, usize)>> {
    let mut possible_non_matching_pairs: Vec<(usize, usize)> = vec![];
    for (index, c_left) in left.into_iter().rev().enumerate() {
        if index >= right.len() {
            continue;
        };

        let c_right = right[index];
        if *c_left != c_right {
            possible_non_matching_pairs.push((left.len() - 1 - index, index + left.len()));
        }
        if possible_non_matching_pairs.len() >= 2 {
            return None;
        }
    }
    Some(possible_non_matching_pairs)
}

fn check_if_outlier_pair_is_smudged(lines: &Vec<String>, outlier_pair: (usize, usize)) -> bool {
    let (left_outlier, right_outlier) = outlier_pair;

    let left: Vec<char> = lines[left_outlier].chars().collect();
    let right: Vec<char> = lines[right_outlier].chars().collect();

    let mut number_of_non_matching = 0;
    for index in 0..left.len() {
        if left[index] != right[index] {
            number_of_non_matching += 1;
        }
        if number_of_non_matching >= 2 {
            return false;
        }
    }
    true
}

fn do_overlap(row: &Vec<usize>, i: usize, lines: &Vec<String>) -> bool {
    let left: Vec<usize> = row[0..i].to_vec();
    let right: Vec<usize> = row[i..row.len()].to_vec();

    match does_contain_each_other_possibly(&left, &right) {
        Some(non_matching_pairs) => match non_matching_pairs.first() {
            Some(outlier_pair) => check_if_outlier_pair_is_smudged(lines, *outlier_pair),
            None => false,
        },
        None => false,
    }
}

fn get_col_of_reflection(col: &Vec<usize>, lines: &Vec<String>) -> usize {
    let rows_of_reflection: Vec<usize> = (1..col.len())
        .into_iter()
        .filter(|row_index| do_overlap(col, *row_index, lines))
        .map(|row_index| row_index)
        .collect();

    // println!("rows_of_reflection {:?}", rows_of_reflection);
    match rows_of_reflection.first() {
        Some(index) => index.clone(),
        None => 0,
    }
}

fn get_row_of_reflection(row: &Vec<usize>, lines: &Vec<String>) -> usize {
    get_col_of_reflection(row, lines) * 100
}

pub fn main() {
    println!("PART 2 ------------");

    let input = read_text("..\\Data\\input_1.txt".to_string());
    let springs = parse_input(&input);

    let sum: usize = springs
        .into_iter()
        .map(|spring| {
            get_row_of_reflection(&spring.row_hashs, &spring._lines)
                + get_col_of_reflection(&spring.col_hashs, &spring._lines_transposed)
        })
        .sum();

    println!("sum = {sum}");
}
