use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
};

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

fn get_non_visited_maze(input: &String) -> Vec<Vec<i128>> {
    input
        .split("\r\n")
        .into_iter()
        .map(|line| line.chars().into_iter().map(|_| i128::MAX).collect())
        .collect()
}

fn set_visited(visited_maze: &mut Vec<Vec<i128>>, bunny_pos: &(usize, usize), distance: i128) {
    let (y, x) = bunny_pos;
    visited_maze[*y][*x] = std::cmp::min(visited_maze[*y][*x], distance);
    // print_visited_maze(&visited_maze);
}

fn find_bunny_position(maze: &Vec<Vec<char>>) -> (usize, usize) {
    for y in 0..maze.len() {
        for x in 0..maze[y].len() {
            let c = &maze[y][x];
            match c {
                'S' => return (y, x),
                _ => continue,
            }
        }
    }
    panic!("Didnt find bunny")
}

fn is_valid_pos(
    maze: &Vec<Vec<char>>,
    direction: &&str,
    pos: &(i128, i128),
    current_char: char,
) -> bool {
    let pipe_to_direction: HashMap<char, &str> = HashMap::from([
        ('|', "north_south"),
        ('-', "east_west"),
        ('L', "north_east"),
        ('J', "north_west"),
        ('7', "south_west"),
        ('F', "south_east"),
        ('S', "north_south_east_west"),
    ]);

    if !pipe_to_direction[&current_char].contains(direction) {
        return false;
    }

    let (y, x) = pos;

    let is_y_ok = *y >= 0 && *y < maze.len() as i128;
    if !is_y_ok {
        return false;
    }

    let is_x_ok = *x >= 0 && *x < maze[*y as usize].len() as i128;
    if !is_x_ok {
        return false;
    }

    let c = &maze[*y as usize][*x as usize];

    let allowed_ways: HashMap<&str, &str> = HashMap::from([
        ("north", "|7F"), // ↑ ↰ ↱
        ("west", "-FL"),  // ← v ^
        ("east", "-J7"),  // → ↴ ^
        ("south", "|LJ"), // ↓ ↳ ↲
    ]);

    allowed_ways[direction].contains(*c)
}

fn find_possible_next_ways(
    maze: &Vec<Vec<char>>,
    bunny_pos: &(usize, usize),
) -> Vec<(usize, usize)> {
    let (bunny_pos_y_u, bunny_pos_x_u) = bunny_pos;
    let bunny_pos_y = *bunny_pos_y_u as i128;
    let bunny_pos_x = *bunny_pos_x_u as i128;

    let possible_way: HashMap<&str, (i128, i128)> = HashMap::from([
        ("north", (bunny_pos_y - 1, bunny_pos_x)), //   "|7F"), // ↑ ↰ ↱
        ("west", (bunny_pos_y, bunny_pos_x - 1)),  //   "-FL"),  // ← v ^
        ("east", (bunny_pos_y, bunny_pos_x + 1)),  //   "-J7"),  // → ↴ ^
        ("south", (bunny_pos_y + 1, bunny_pos_x)), //   "|LJ"), // ↓ ↳ ↲
    ]);

    let current_char = maze[*bunny_pos_y_u][*bunny_pos_x_u];
    let ways: Vec<(usize, usize)> = possible_way
        .into_iter()
        .filter(|(direction, pos)| is_valid_pos(maze, direction, pos, current_char))
        .map(|(_, pos)| {
            let (y, x) = pos;
            (y as usize, x as usize)
        })
        .collect();

    ways
}

fn move_bunny(
    maze: &Vec<Vec<char>>,
    bunny_pos: &(usize, usize),
    visited_maze: &mut Vec<Vec<i128>>,
    distance: i128,
) -> Vec<(usize, usize)> {
    set_visited(visited_maze, bunny_pos, distance);

    let possible_entrances = find_possible_next_ways(&maze, &bunny_pos);

    possible_entrances
        .into_iter()
        .filter(|entrance_pos| {
            let (y, x) = entrance_pos;
            let upcomming_distance = visited_maze[*y][*x];

            let next_distance = distance + 1;
            next_distance < upcomming_distance
        })
        .collect()
}

fn check_all_entrances(
    mut entrance_deque: VecDeque<(i128, (usize, usize))>,
    maze: &Vec<Vec<char>>,
    visited_maze: &mut Vec<Vec<i128>>,
) {
    while !entrance_deque.is_empty() {
        let (distance, possible_entrance) = entrance_deque.pop_front().unwrap();

        let possible_new_entrances: Vec<(usize, usize)> =
            move_bunny(&maze, &possible_entrance, visited_maze, distance);

        let new_distance = distance + 1;
        for possible_new_entrance in possible_new_entrances {
            entrance_deque.push_back((new_distance, possible_new_entrance));
        }
    }
}

fn get_start_entrances(
    maze: &Vec<Vec<char>>,
    bunny_pos: (usize, usize),
    start_pos: i128,
) -> VecDeque<(i128, (usize, usize))> {
    let mut entrance_deque = VecDeque::new();
    for possible_entrance in find_possible_next_ways(maze, &bunny_pos) {
        entrance_deque.push_back((start_pos + 1, possible_entrance));
    }
    entrance_deque
}

fn print_visited_maze(visited_maze: &Vec<Vec<i128>>) {
    println!("print_visited_maze: --------------------------------");
    for (i, line) in visited_maze.iter().enumerate() {
        print!("{i} \t");
        for num in line {
            match *num {
                i128::MAX => print!("-"),
                _ => print!("0"),
            }
        }
        print!("\n");
    }
    print!("\n");
}

fn print_is_inside_maze(visited_maze: &Vec<Vec<i128>>) {
    println!("print_is_inside_maze: --------------------------------");
    for (i, line) in visited_maze.iter().enumerate() {
        print!("{i} \t");
        for num in line {
            match *num {
                -1 => print!("-"),
                num => print!("{num}"),
            }
        }
        print!("\n");
    }
    print!("\n");
}

fn print_visited(visited_maze: &Vec<Vec<i128>>) {
    println!("print_visited: --------------------------------");
    for (i, line) in visited_maze.iter().enumerate() {
        print!("{i} \t");
        for num in line {
            print!("{num}")
        }
        print!("\n");
    }
    print!("\n");
}

fn is_inside_fn(num: &i128, c: &char, is_inside: &mut bool) -> i128 {
    let is_on_line = match *num {
        i128::MAX => false,
        _ => true,
    };

    // let does_open = "SF|7".contains(c);
    // let does_close = "S7|F".contains(c);
    // let does_not_open = "LJ-".contains(c);
    // let does_not_close = "LJ-".contains(c);

    // ("north", "|7F"), // ↑ ↰ ↱
    // ("west", "-FL"),  // ← v ^
    // ("east", "-J7"),  // → ↴ ^
    // ("south", "|LJ"), // ↓ ↳ ↲

    match is_on_line {
        true => {
            if "F|7".contains(*c) {
                *is_inside = !*is_inside;
            }
            -1
        }
        false => is_inside.clone() as i128,
    }

    // if "SF|7".contains(*c) {
    //     // print!("{is_inside} -> ");
    //     *is_inside = match is_on_line {
    //         true => !*is_inside,
    //         false => *is_inside,
    //     };
    //     // print!("{is_inside} \n");
    // } else {
    //     // print!("..\n");
    // }

    // if is_on_line {
    //     return -1;
    // }

    // return is_inside.clone() as i128;
}

fn get_in_out_maze(maze: &Vec<Vec<char>>, visited_maze: &Vec<Vec<i128>>) -> Vec<Vec<i128>> {
    visited_maze
        .iter()
        .enumerate()
        .map(|(i, _)| {
            // println!("{i}---------");
            let mut is_inside = false;

            let line: Vec<i128> = maze[i]
                .iter()
                .zip(visited_maze[i].iter())
                .map(|(c, num)| {
                    // println!("is_inside : {is_inside}");
                    is_inside_fn(num, c, &mut is_inside)
                })
                .collect();
            line
        })
        .collect()
}

fn flood(
    y: i128,
    x: i128,
    in_out_maze: &Vec<Vec<i128>>,
    flooded_maze: &mut Vec<Vec<i128>>,
    index: i128,
) {
    let possible_nexts = vec![(y - 1, x), (y, x - 1), (y, x + 1), (y + 1, x)];
    let real_nexts: Vec<(usize, usize)> = possible_nexts
        .iter()
        .filter(|(y, x)| {
            let is_y_ok = *y >= 0 && *y < in_out_maze.len() as i128;
            match is_y_ok {
                true => {
                    let is_x_ok = *x >= 0 && *x < in_out_maze[*y as usize].len() as i128;
                    is_x_ok
                }
                false => false,
            }
        })
        .map(|(y, x)| (*y as usize, *x as usize))
        .collect();

    for next in real_nexts {
        let (yy, xx) = next;

        let is_island = in_out_maze[yy][xx] == 1;
        let was_visited = flooded_maze[yy][xx] != 0;

        if is_island && !was_visited {
            flooded_maze[yy][xx] = index;
            flood(yy as i128, xx as i128, in_out_maze, flooded_maze, index);
        }
    }
}

fn get_empty_flood_maze(in_out_maze: &Vec<Vec<i128>>) -> Vec<Vec<i128>> {
    in_out_maze
        .iter()
        .map(|line| line.iter().map(|_| 0).collect())
        .collect()
}

fn flood_fill(in_out_maze: &Vec<Vec<i128>>) -> Vec<Vec<i128>> {
    let mut i = 0;
    let mut flooded_maze = get_empty_flood_maze(in_out_maze);
    print_visited(&flooded_maze);

    for y in 0..in_out_maze.len() as i128 {
        for x in 0..in_out_maze[y as usize].len() as i128 {
            flood(y, x, in_out_maze, &mut flooded_maze, i);
            i = i + 1;
        }
    }
    flooded_maze
}

pub fn main() {
    println!("PART 2 ------------");

    // let input = read_text("..\\Data\\test_input_4.txt".to_string());
    let input = read_text("..\\Data\\input_1.txt".to_string());
    let maze = parse_input(&input);
    let mut visited_maze = get_non_visited_maze(&input);
    let bunny_pos = find_bunny_position(&maze);

    let start_pos = 0;
    set_visited(&mut visited_maze, &bunny_pos, start_pos);

    let entrance_deque = get_start_entrances(&maze, bunny_pos, start_pos);
    check_all_entrances(entrance_deque, &maze, &mut visited_maze);

    print_visited_maze(&visited_maze);
    let in_out_maz = get_in_out_maze(&maze, &visited_maze);
    print_is_inside_maze(&in_out_maz);

    let flooded_maze = flood_fill(&in_out_maz);
    print_visited(&flooded_maze);

    let num_flooded: i128 = flooded_maze
        .clone()
        .into_iter()
        .flatten()
        .filter(|num| *num != 0)
        .map(|_| 1)
        .sum();
    println!("num num_flooded: {num_flooded}");

    let islands: HashSet<i128> = HashSet::from_iter(flooded_maze.iter().flatten().cloned());
    let len = islands.len() as i128 - 1;
    println!("num diff_islands: {len}");
}
