use std::{collections::HashMap, fs};

fn read_text(file_path: String) -> String {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    return contents;
}

#[derive(Debug)]
pub struct Directions {
    left: String,
    right: String,
}

impl Directions {
    pub fn create(line: &str) -> Directions {
        let mut split = line.split(", ");
        let left: &str = split.next().unwrap();
        let right = split.next().unwrap();

        let left_first_off: &str = &left[1..left.len()];
        let right_last_off: &str = &right[0..right.len() - 1];

        Directions {
            left: left_first_off.to_string(),
            right: right_last_off.to_string(),
        }
    }

    pub fn go(&self, direction: &char) -> String {
        match direction {
            'R' => self.right.clone(),
            'L' => self.left.clone(),
            _ => panic!("Direcetion invalid: {direction}"),
        }
    }
}

fn parse_input(input: String) -> (Vec<char>, HashMap<String, Directions>) {
    let mut split = input.split("\r\n\r\n");
    let instructions = split.next().unwrap();
    let body = split.next().unwrap();

    let network: HashMap<String, Directions> = body
        .split("\r\n")
        .into_iter()
        .map(|line| {
            let mut line_split = line.split(" = ");

            let node = line_split.next().unwrap();
            let node_body = line_split.next().unwrap();
            (node.to_string(), Directions::create(node_body))
        })
        .collect();

    (instructions.chars().into_iter().collect(), network)
}

fn get_ending_with_a(network: &HashMap<String, Directions>) -> Vec<String> {
    network
        .iter()
        .filter(|(k, _)| k.chars().last().unwrap() == 'A')
        .map(|(k, _)| k.clone())
        .collect()
}

fn does_end_with_z(node: &String) -> bool {
    node.chars().into_iter().last().unwrap() == 'Z'
}

fn iterate_until_end_with_z(
    node: &String,
    directions: &Vec<char>,
    network: &HashMap<String, Directions>,
) -> u128 {
    let mut steps = 0;
    let mut current = node.clone();
    while !does_end_with_z(&current) {
        let current_direction = directions.get(steps % directions.len()).unwrap();
        let current_node = network.get(&current).unwrap();

        current = current_node.go(current_direction);
        steps = steps + 1;
    }
    steps as u128
}

fn find_smallest_common(end_steps: Vec<u128>) -> u128 {
    let mut smallest_common_mult: u128 = 1;
    for num in end_steps.iter() {
        smallest_common_mult = num::integer::lcm(smallest_common_mult, num.clone());
    }
    smallest_common_mult
}

pub fn main() {
    println!("PART 2 ------------");
    let input = read_text("..\\Data\\input_1.txt".to_string());

    let (directions, network) = parse_input(input);

    let start_nodes = get_ending_with_a(&network);
    let end_steps: Vec<u128> = start_nodes
        .iter()
        .map(|start_node| iterate_until_end_with_z(start_node, &directions, &network))
        .collect();

    let smallest_common_mult = find_smallest_common(end_steps);
    println!("smallest_common_mult: {smallest_common_mult}");
}
