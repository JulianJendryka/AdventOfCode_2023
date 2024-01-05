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

fn find_empty_rows(galaxies: &Vec<Vec<char>>) -> Vec<usize> {
    galaxies
        .into_iter()
        .enumerate()
        .filter(|(_, row)| row.into_iter().all(|c| *c == '.'))
        .map(|(row_index, _)| row_index)
        .collect()
}

fn transpose(galaxies: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let rows = galaxies.len();
    let cols = galaxies[0].len();

    (0..cols)
        .map(|col| (0..rows).map(|row| galaxies[row][col]).collect())
        .collect()
}

fn get_expanded_space(galaxies: &Vec<Vec<char>>) -> (Vec<usize>, Vec<usize>) {
    let empty_rows = find_empty_rows(&galaxies);
    let empty_cols = find_empty_rows(&transpose(galaxies));
    (empty_rows, empty_cols)
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

fn get_expansion_factor(empty_space_location: &Vec<usize>, pos: &usize) -> usize {
    let mut expansion_factor = 0;
    while expansion_factor < empty_space_location.len()
        && *pos > empty_space_location[expansion_factor]
    {
        expansion_factor += 1;
    }
    expansion_factor
}

fn get_expanded_pos(pos: &(usize, usize), expansion: &(Vec<usize>, Vec<usize>)) -> (usize, usize) {
    let (empty_rows, empty_cols) = expansion;

    let (y, x) = pos;

    let expansion_factor_y = get_expansion_factor(empty_rows, y);
    let expansion_factor_x = get_expansion_factor(empty_cols, x);

    let expansion = 1000000 - 1;
    (
        y + expansion_factor_y * expansion,
        x + expansion_factor_x * expansion,
    )
}

fn get_expanded_galaxy_locations(
    galaxy_locations: &HashMap<usize, (usize, usize)>,
    expansion: (Vec<usize>, Vec<usize>),
) -> HashMap<usize, (usize, usize)> {
    galaxy_locations
        .into_iter()
        .map(|(galaxy_id, position)| (*galaxy_id, get_expanded_pos(position, &expansion)))
        .collect()
}

fn calculate_manhatten_distance(pos1: &(usize, usize), pos2: &(usize, usize)) -> i64 {
    let (y1, x1) = pos1;
    let (y2, x2) = pos2;

    (*y1 as i64 - *y2 as i64).abs() + (*x1 as i64 - *x2 as i64).abs()
}

fn _print_sorted(locations: &HashMap<usize, (usize, usize)>) {
    let mut sorted: Vec<_> = locations.iter().collect();
    sorted.sort_by_key(|(key, _)| **key);

    for (key, value) in sorted.iter() {
        println!("{} = {:?}", key, value);
    }
}

pub fn main() {
    println!("PART 2 ------------");

    let input = read_text("..\\Data\\input_1.txt".to_string());
    let galaxies = parse_input(&input);
    let expansion = get_expanded_space(&galaxies);

    let galaxy_locations = get_galaxy_locations(&galaxies);
    let expanded_galaxy_locations = get_expanded_galaxy_locations(&galaxy_locations, expansion);

    let pairings = get_parings(expanded_galaxy_locations.keys().collect());

    let distances: Vec<i64> = pairings
        .into_iter()
        .map(|(galaxy_a, galaxy_b)| {
            calculate_manhatten_distance(
                expanded_galaxy_locations.get(&galaxy_a).unwrap(),
                expanded_galaxy_locations.get(&galaxy_b).unwrap(),
            )
        })
        .collect();

    let sum: i64 = distances.into_iter().sum();
    println!("sum = {:?}", sum);
}
