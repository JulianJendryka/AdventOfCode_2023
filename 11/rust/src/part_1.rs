use std::{collections::HashMap, fs};

fn read_text(file_path: String) -> String {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    return contents;
}

fn parse_input(input: &String) -> Vec<Vec<char>> {
    input
        .split("\r\n")
        .into_iter()
        .map(|line| line.chars().into_iter().map(|c| c.clone()).collect())
        .collect()
}

fn print_galaxies(galaxies: &Vec<Vec<char>>) {
    println!("print_galaxies: --------------------------------");
    for (i, line) in galaxies.iter().enumerate() {
        print!("{i} \t");
        for c in line {
            print!("{c}")
        }
        print!("\n");
    }
    print!("\n");
}

fn find_empty_rows(galaxies: &Vec<Vec<char>>) -> Vec<usize> {
    galaxies
        .into_iter()
        .enumerate()
        .filter(|(_, row)| row.into_iter().all(|c| *c == '.'))
        .map(|(row_index, _)| row_index)
        .collect()
}

fn insert_empty_rows(galaxies: &mut Vec<Vec<char>>, rows: Vec<usize>) {
    let empty_row = galaxies[rows[0]].clone();
    for (num_row_to_be_iserted, row_index) in rows.iter().enumerate() {
        galaxies.insert(*row_index + num_row_to_be_iserted, empty_row.clone());
    }
}

fn find_and_emplace_empty_rows(galaxies: &mut Vec<Vec<char>>) {
    let empty_rows = find_empty_rows(&galaxies);
    match !empty_rows.is_empty() {
        true => insert_empty_rows(galaxies, empty_rows),
        false => (),
    }
}

fn transpose(galaxies: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rows = galaxies.len();
    let cols = galaxies[0].len();

    (0..cols)
        .map(|col| (0..rows).map(|row| galaxies[row][col]).collect())
        .collect()
}
fn find_and_emplace_empty_cols(galaxies: &mut Vec<Vec<char>>) {
    let mut transposed_galaxies = transpose(galaxies);

    find_and_emplace_empty_rows(&mut transposed_galaxies);
    *galaxies = transpose(&transposed_galaxies);
}

fn expand_space(galaxies: &mut Vec<Vec<char>>) {
    find_and_emplace_empty_rows(galaxies);
    find_and_emplace_empty_cols(galaxies);
}

fn get_galaxy_locations(galaxies: &Vec<Vec<char>>) -> HashMap<usize, (usize, usize)> {
    let mut index = 0;
    galaxies
        .into_iter()
        .enumerate()
        .map(|(y, row)| {
            let row_map: HashMap<usize, (usize, usize)> = row
                .into_iter()
                .enumerate()
                .filter(|(_, c)| **c == '#')
                .map(|(x, _)| {
                    index += 1;
                    (index, (y, x))
                })
                .collect();
            row_map
        })
        .flatten()
        .collect()
}

fn get_parings(galaxy_namess: Vec<&usize>) -> Vec<(usize, usize)> {
    let mut galaxy_names = galaxy_namess.clone();

    let mut pairings: Vec<(usize, usize)> = vec![];

    while !galaxy_names.is_empty() {
        let galaxy_name = &galaxy_names.pop().unwrap();

        let mut galaxy_pairing: Vec<(usize, usize)> = galaxy_names
            .iter()
            .map(|other_galaxy_name| (**galaxy_name, **other_galaxy_name))
            .collect();

        pairings.append(&mut galaxy_pairing);
    }
    pairings
}
fn calculate_manhatten_distance(pos1: &(usize, usize), pos2: &(usize, usize)) -> i64 {
    let (y1, x1) = pos1;
    let (y2, x2) = pos2;

    (*y1 as i64 - *y2 as i64).abs() + (*x1 as i64 - *x2 as i64).abs()
}
pub fn main() {
    println!("PART 1 ------------");

    let input = read_text("..\\Data\\input_1.txt".to_string());
    let mut galaxies = parse_input(&input);
    expand_space(&mut galaxies);
    print_galaxies(&galaxies);

    let galaxy_locations = get_galaxy_locations(&galaxies);

    let pairings = get_parings(galaxy_locations.keys().collect());

    let distances: Vec<i64> = pairings
        .into_iter()
        .map(|(galaxy_a, galaxy_b)| {
            calculate_manhatten_distance(
                galaxy_locations.get(&galaxy_a).unwrap(),
                galaxy_locations.get(&galaxy_b).unwrap(),
            )
        })
        .collect();

    let sum: i64 = distances.into_iter().sum();
    println!("sum = {:?}", sum);
}
