use std::{fs, usize};

fn read_text(file_path: String) -> String {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    return contents;
}

#[derive(Clone, Debug)]
pub struct Edge {
    direction: char,
    y: usize,
    x: usize,
}

impl Edge {
    pub fn create(direction: char, y: usize, x: usize) -> Edge {
        Edge { direction, y, x }
    }
}

#[derive(Clone, Debug)]
struct Dig {
    direction: char,
    lenght: usize,
    _color: String,
}

impl Dig {
    pub fn create(line: String) -> Dig {
        let mut split = line.split(" ");

        let left: Vec<char> = split.next().unwrap().chars().collect();
        let middle = split.next().unwrap();
        let right = split.next().unwrap();

        Dig {
            direction: left[0],
            lenght: middle.parse::<usize>().unwrap(),
            _color: right.to_string(),
        }
    }
}

fn parse_input(input: String) -> Vec<Dig> {
    input
        .split("\n")
        .into_iter()
        .map(|line| Dig::create(line.to_string()))
        .collect()
}

fn get_dug_coords(dig_instructions: Vec<Dig>) -> (Vec<Edge>, (usize, usize)) {
    let mut dug_coords: Vec<(char, i32, i32)> = vec![];

    let (mut min_y, mut min_x, mut max_y, mut max_x) = (0, 0, 0, 0);

    let (mut current_y, mut current_x) = (0, 0);
    for instruction in dig_instructions {
        let (delta_y, delta_x) = match instruction.direction {
            'U' => (-1, 0),
            'D' => (1, 0),
            'L' => (0, -1),
            'R' => (0, 1),
            _ => panic!("Unknown direction"),
        };

        for _ in 0..instruction.lenght {
            current_y += delta_y;
            current_x += delta_x;
            dug_coords.push((instruction.direction, current_y, current_x));
        }

        min_y = std::cmp::min(min_y, current_y);
        max_y = std::cmp::max(max_y, current_y);

        min_x = std::cmp::min(min_x, current_x);
        max_x = std::cmp::max(max_x, current_x);
    }

    for (_, y, x) in dug_coords.iter_mut() {
        *y -= min_y;
        *x -= min_x;
    }
    max_y -= min_y;
    max_x -= min_x;

    let first_dir = dug_coords[0].0;

    for i in 0..dug_coords.len() {
        let next_dir = match i as i32 == ((dug_coords.len() as i32) - 1) {
            true => first_dir.clone(),
            false => dug_coords[i + 1].0.clone(),
        };

        let x = match dug_coords[i].0 {
            'U' => match next_dir {
                'U' => '|',
                'D' => '|',
                'L' => '7',
                'R' => 'F',
                _ => panic!("Unknown direction"),
            },
            'D' => match next_dir {
                'U' => '|',
                'D' => '|',
                'L' => 'J',
                'R' => 'L',
                _ => panic!("Unknown direction"),
            },
            'L' => match next_dir {
                'U' => 'L',
                'D' => 'F',
                'L' => '-',
                'R' => '-',
                _ => panic!("Unknown direction"),
            },
            'R' => match next_dir {
                'U' => 'J',
                'D' => '7',
                'L' => '-',
                'R' => '-',
                _ => panic!("Unknown direction"),
            },
            _ => panic!("Unknown direction"),
        };

        dug_coords[i].0 = x;
    }
    // dug_coords[0].0 = 'S';

    let x = dug_coords
        .into_iter()
        .map(|(dir, y, x)| Edge::create(dir, y as usize, x as usize))
        .collect();

    (x, ((max_y + 1) as usize, (max_x + 1) as usize))
}

#[derive(Clone, Debug)]
pub enum Elements {
    Border,
    Inside(bool),
    _Filled(usize),
    Digged(Edge),
    Empty,
}

pub struct Field {
    y_size: usize,
    x_size: usize,
    field: Vec<Vec<Elements>>,
}

impl Field {
    pub fn create(all_edges: Vec<Edge>, size: (usize, usize)) -> Field {
        let (y, x) = size;
        let mut field = Field::create_empty(y + 2, x + 2);

        for edge in all_edges {
            let new_y = edge.y + 1;
            let new_x = edge.x + 1;
            let new_edge = Edge::create(edge.direction, new_y, new_x);

            field.field[new_y][new_x] = Elements::Digged(new_edge);
        }

        field
    }

    fn create_empty(y_size: usize, x_size: usize) -> Field {
        let mut field = Field::create_blank(y_size, x_size);
        field.set_border();
        field
    }

    fn create_blank(y_size: usize, x_size: usize) -> Field {
        Field {
            y_size: y_size,
            x_size: x_size,
            field: (0..y_size + 2)
                .into_iter()
                .map(|_| {
                    (0..x_size + 2)
                        .into_iter()
                        .map(|_| Elements::Empty)
                        .collect()
                })
                .collect(),
        }
    }

    fn set_border(&mut self) {
        for y in 0..self.y_size {
            for x in 0..self.x_size {
                if (x == 0 || x == self.x_size - 1) || (y == 0 || y == self.y_size - 1) {
                    self.field[y][x] = Elements::Border;
                    continue;
                }
            }
        }
    }

    fn _print(&self) {
        for y in 0..self.y_size {
            for x in 0..self.x_size {
                match &self.field[y][x] {
                    Elements::Border => print!("X "),
                    Elements::Inside(is_inside) => match is_inside {
                        true => print!("0 "),
                        false => print!(". "),
                    },
                    Elements::_Filled(index) => print!("{index} "),
                    Elements::Digged(edge) => print!("{0} ", edge.direction),
                    Elements::Empty => print!(". "),
                }
            }
            print!("\n");
        }
    }

    fn count_edges(&self) -> usize {
        self.field
            .iter()
            .map(|row| {
                row.iter()
                    .map(|element| match element {
                        Elements::Border => 0,
                        Elements::Inside(_) => 0,
                        Elements::_Filled(_) => 0,
                        Elements::Digged(_) => 1,
                        Elements::Empty => 0,
                    })
                    .sum::<usize>()
            })
            .sum()
    }

    fn count_dug(&self) -> usize {
        self.field
            .iter()
            .map(|row| {
                row.iter()
                    .map(|element| match element {
                        Elements::Border => 0,
                        Elements::Inside(is_inside) => match is_inside {
                            true => 1,
                            false => 0,
                        },
                        Elements::_Filled(_) => 0,
                        Elements::Digged(_) => 1,
                        Elements::Empty => 0,
                    })
                    .sum::<usize>()
            })
            .sum()
    }

    fn _flood_fill(&mut self) {
        let mut i = 0;

        for y in 0..self.field.len() as i128 {
            for x in 0..self.field[y as usize].len() as i128 {
                self._flood(y, x, i);
                i = i + 1;
            }
        }
    }

    fn _flood(&mut self, y: i128, x: i128, index: usize) {
        let possible_nexts = vec![(y - 1, x), (y, x - 1), (y, x + 1), (y + 1, x)];
        let real_nexts: Vec<(usize, usize)> = possible_nexts
            .iter()
            .filter(|(y, x)| {
                let is_y_ok = *y >= 0 && *y < self.field.len() as i128;
                match is_y_ok {
                    true => {
                        let is_x_ok = *x >= 0 && *x < self.field[*y as usize].len() as i128;
                        is_x_ok
                    }
                    false => false,
                }
            })
            .map(|(y, x)| (*y as usize, *x as usize))
            .collect();

        for next in real_nexts {
            let (yy, xx) = next;

            match self.field[yy][xx] {
                Elements::_Filled(_) => (),
                Elements::Inside(_) => (),
                Elements::Border => (),
                Elements::Digged(_) => (),
                Elements::Empty => {
                    self.field[yy][xx] = Elements::_Filled(index);
                    self._flood(yy as i128, xx as i128, index);
                }
            }
        }
    }

    fn get_in_out(&mut self) {
        for row in self.field.iter_mut() {
            let mut is_inside = false;

            for el in row.iter_mut() {
                Field::is_inside_fn(el, &mut is_inside);
            }
        }
    }

    fn is_inside_fn(element: &mut Elements, is_inside: &mut bool) {
        match element {
            Elements::Digged(edge) => {
                if "SF|7".contains(edge.direction) {
                    *is_inside = !*is_inside;
                }
            }
            _ => *element = Elements::Inside(is_inside.clone()),
        }
    }
}

pub fn main() {
    println!("PART 1 ------------");
    let input = read_text("..\\Data\\test_input_1.txt".to_string());

    let dig_instructions = parse_input(input);

    let (dug_coord, size) = get_dug_coords(dig_instructions);

    let mut field = Field::create(dug_coord, size);

    let num_lava_edges = field.count_edges();
    println!("num_lava_edges = {num_lava_edges}");

    // field._print();

    field.get_in_out();
    field._print();

    let count_dug = field.count_dug();
    println!("count_dug = {count_dug}");
}
