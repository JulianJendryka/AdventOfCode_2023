use std::fs;

fn read_text(file_path: String) -> String {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    return contents;
}

#[derive(Clone, Debug)]
struct Dig {
    direction: char,
    lenght: i128,
    _color: String,
}

impl Dig {
    pub fn create(line: String) -> Dig {
        let mut split = line.split(" ");

        let _left: Vec<char> = split.next().unwrap().chars().collect();
        let _middle = split.next().unwrap();

        let right = split.next().unwrap();
        let hexa: Vec<char> = right.chars().collect();

        let hexa_direction = hexa[hexa.len() - 2];
        let hexa_length: Vec<char> = hexa[2..=6].to_vec();
        let hexa_length_string: String = hexa_length.iter().collect();
        let hexa_num = Dig::decode_hexa(hexa_length_string);
        // println!("{hexa_num}");

        Dig {
            direction: match hexa_direction {
                '0' => 'R',
                '1' => 'D',
                '2' => 'L',
                '3' => 'U',
                _ => panic!("unknown dir"),
            },
            lenght: hexa_num,
            _color: right.to_string(),
        }
    }

    fn decode_hexa(hexa_string: String) -> i128 {
        let chars = hexa_string.chars();

        chars
            .into_iter()
            .rev()
            .enumerate()
            .map(|(index, char)| {
                let num = match char {
                    '0' => 0,
                    '1' => 1,
                    '2' => 2,
                    '3' => 3,
                    '4' => 4,
                    '5' => 5,
                    '6' => 6,
                    '7' => 7,
                    '8' => 8,
                    '9' => 9,
                    'a' => 10,
                    'b' => 11,
                    'c' => 12,
                    'd' => 13,
                    'e' => 14,
                    'f' => 15,
                    _ => panic!("unknown dir"),
                };
                num * (16 as i128).pow(index as u32)
            })
            .sum()
    }
}

fn parse_input(input: String) -> Vec<Dig> {
    input
        .split("\n")
        .into_iter()
        .map(|line| Dig::create(line.to_string()))
        .collect()
}

#[derive(Clone, Debug)]
pub struct Field {
    umfang: i128,
    corner: Vec<(i128, i128)>,
}

impl Field {
    fn create(dig_instructions: Vec<Dig>) -> Field {
        let mut field = Field::create_empty();

        let (mut y, mut x) = (0, 0);
        for instruction in dig_instructions {
            field.umfang += instruction.lenght;
            field.corner.push((y, x));

            let (delta_y, delta_x) = match instruction.direction {
                'U' => (-1i128, 0i128),
                'D' => (1, 0),
                'L' => (0, -1),
                'R' => (0, 1),
                _ => panic!("Unknown direction"),
            };

            // println!("{y} : {x}");
            (y, x) = (
                (y as i128 + delta_y * instruction.lenght as i128) as i128,
                (x as i128 + delta_x * instruction.lenght as i128) as i128,
            );
        }
        field
    }

    fn create_empty() -> Field {
        Field {
            umfang: 0,
            corner: vec![],
        }
    }

    fn get_area(&self) -> i128 {
        let mut next_corner = self.corner.clone();
        next_corner.push(self.corner.first().unwrap().clone());

        let mut sum = 0;
        for i in 0..next_corner.len() {
            let current_index = match i == 0 {
                true => next_corner.len() - 1,
                false => i - 1,
            };
            let current = next_corner[current_index];
            let next = next_corner[i];

            sum += ((current.0 + next.0) * (current.1 - next.1)) / 2;
        }

        sum.abs() + self.umfang / 2 + 1
    }
}

pub fn main() {
    println!("PART 2 ------------");
    let input = read_text("..\\Data\\input_1.txt".to_string());

    let dig_instructions = parse_input(input);

    let field = Field::create(dig_instructions);

    let area = field.get_area();
    println!("area = {area}")
}
