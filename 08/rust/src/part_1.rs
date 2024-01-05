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

pub fn main() {
    println!("PART 1 ------------");
    let input = read_text("..\\Data\\input_1.txt".to_string());
    let (directions, network) = parse_input(input);

    let mut steps = 0;
    let mut current = "AAA".to_string();
    while current != "ZZZ".to_string() {
        let current_direction = directions.get(steps % directions.len()).unwrap();
        let current_node = network.get(&current).unwrap();

        current = current_node.go(current_direction);
        steps = steps + 1;
    }

    println!("total steps {steps}");
}
