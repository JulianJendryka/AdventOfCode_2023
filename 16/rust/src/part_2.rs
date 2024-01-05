use std::{collections::VecDeque, fs};

fn read_text(file_path: String) -> String {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    return contents;
}

fn parse_input(input: String) -> Vec<Vec<char>> {
    input
        .split("\n")
        .into_iter()
        .map(|line| line.chars().collect())
        .collect()
}

fn get_visited_board(layout: &Vec<Vec<char>>) -> Vec<Vec<Vec<char>>> {
    layout
        .into_iter()
        .map(|line| line.into_iter().map(|_| vec![]).collect())
        .collect()
}

pub fn _clear_terminal_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

    // print!("{}[2J", 27 as char);

    // if cfg!(target_os = "windows") {
    //     Command::new("cmd")
    //         .args(["/c", "cls"])
    //         .spawn()
    //         .expect("cls command failed to start")
    //         .wait()
    //         .expect("failed to wait");
    // } else {
    //     Command::new("clear")
    //         .spawn()
    //         .expect("clear command failed to start")
    //         .wait()
    //         .expect("failed to wait");
    // };
}

fn _print_board_visited(layout: &Vec<Vec<Vec<char>>>) {
    println!("------------");
    for y in 0..layout.len() {
        for x in 0..layout[y].len() {
            match layout[y][x].len() {
                0 => print!("."),
                1 => print!("{0}", layout[y][x][0]),
                _ => print!("2"),
            }
        }
        print!("\n");
    }
    _clear_terminal_screen();
}

fn _print_energized(layout: &Vec<Vec<Vec<char>>>) {
    println!("------------");
    for y in 0..layout.len() {
        for x in 0..layout[y].len() {
            match layout[y][x].len() {
                0 => print!("."),
                _ => print!("#"),
            }
        }
        print!("\n");
    }
}

fn get_next_pos(
    layout: &Vec<Vec<char>>,
    visited_board: &mut Vec<Vec<Vec<char>>>,
    pos_moved_on: (usize, usize),
    last_symbol: char,
) -> Vec<(usize, usize, char)> {
    let (y_og, x_og) = pos_moved_on;
    visited_board[y_og][x_og].push(last_symbol);

    // print_board_visited(&visited_board);

    let y = y_og as i32;
    let x = x_og as i32;

    let x: Vec<Option<(usize, usize, char)>> = match layout[y_og][x_og] {
        '.' => match last_symbol {
            '>' => vec![check_visited(layout, visited_board, y, x + 1, '>')],
            '<' => vec![check_visited(layout, visited_board, y, x - 1, '<')],
            '^' => vec![check_visited(layout, visited_board, y - 1, x, '^')],
            'v' => vec![check_visited(layout, visited_board, y + 1, x, 'v')],
            _ => panic!("Unkown last_symbol"),
        },
        '\\' => match last_symbol {
            '>' => vec![check_visited(layout, visited_board, y + 1, x, 'v')],
            '<' => vec![check_visited(layout, visited_board, y - 1, x, '^')],
            '^' => vec![check_visited(layout, visited_board, y, x - 1, '<')],
            'v' => vec![check_visited(layout, visited_board, y, x + 1, '>')],
            _ => panic!("Unkown last_symbol"),
        },
        '/' => match last_symbol {
            '>' => vec![check_visited(layout, visited_board, y - 1, x, '^')],
            '<' => vec![check_visited(layout, visited_board, y + 1, x, 'v')],
            '^' => vec![check_visited(layout, visited_board, y, x + 1, '>')],
            'v' => vec![check_visited(layout, visited_board, y, x - 1, '<')],
            _ => panic!("Unkown last_symbol"),
        },
        '|' => match last_symbol {
            '>' => vec![
                check_visited(layout, visited_board, y - 1, x, '^'),
                check_visited(layout, visited_board, y + 1, x, 'v'),
            ],
            '<' => vec![
                check_visited(layout, visited_board, y - 1, x, '^'),
                check_visited(layout, visited_board, y + 1, x, 'v'),
            ],
            '^' => vec![check_visited(layout, visited_board, y - 1, x, '^')],
            'v' => vec![check_visited(layout, visited_board, y + 1, x, 'v')],
            _ => panic!("Unkown last_symbol"),
        },
        '-' => match last_symbol {
            '>' => vec![check_visited(layout, visited_board, y, x + 1, '>')],
            '<' => vec![check_visited(layout, visited_board, y, x - 1, '<')],
            '^' => vec![
                check_visited(layout, visited_board, y, x + 1, '>'),
                check_visited(layout, visited_board, y, x - 1, '<'),
            ],
            'v' => vec![
                check_visited(layout, visited_board, y, x + 1, '>'),
                check_visited(layout, visited_board, y, x - 1, '<'),
            ],
            _ => panic!("Unkown last_symbol"),
        },
        _ => panic!("Unkown symbol"),
    };
    x.into_iter()
        .filter(|f| f.is_some())
        .map(|f| f.unwrap())
        .collect()
}

fn check_visited(
    layout: &Vec<Vec<char>>,
    visited_board: &Vec<Vec<Vec<char>>>,
    y: i32,
    x: i32,
    direction: char,
) -> Option<(usize, usize, char)> {
    if y < 0 || y >= layout.len() as i32 {
        return None;
    }

    if x < 0 || x >= layout[y as usize].len() as i32 {
        return None;
    }

    if visited_board[y as usize][x as usize].contains(&direction) {
        //alrdy travelled to -> stop loop
        return None;
    }

    Some((y as usize, x as usize, direction))
}

fn calc_energized(visited_board: &Vec<Vec<Vec<char>>>) -> i32 {
    visited_board
        .into_iter()
        .map(|line| {
            line.into_iter()
                .map(|field| match field.is_empty() {
                    true => 0,
                    false => 1,
                })
                .sum::<i32>()
        })
        .sum()
}

fn get_start_positions(layout: &Vec<Vec<char>>) -> Vec<(usize, usize, char)> {
    let mut start_positions = vec![];

    for y in 0..layout.len() {
        for x in 0..layout[y].len() {
            if (x == 0 || x == layout[y].len() - 1) || (y == 0 || y == layout.len() - 1) {
                if y == 0 {
                    start_positions.push((y, x, 'v'));
                }
                if y == layout.len() - 1 {
                    start_positions.push((y, x, '^'));
                }

                if x == 0 {
                    start_positions.push((y, x, '>'));
                }
                if x == layout[y].len() - 1 {
                    start_positions.push((y, x, '<'));
                }
            }
        }
    }

    start_positions
}

pub fn solve_and_return_energy_score(
    layout: &Vec<Vec<char>>,
    start_pos: (usize, usize, char),
) -> i32 {
    let mut visited_board = get_visited_board(&layout);

    let mut queued_moves: VecDeque<(usize, usize, char)> = VecDeque::new();
    queued_moves.push_back(start_pos);

    while !queued_moves.is_empty() {
        let current_move = queued_moves.pop_front().unwrap();
        let (y, x, c) = current_move;

        let next_moves = get_next_pos(&layout, &mut visited_board, (y, x), c);
        queued_moves.append(&mut VecDeque::from(next_moves));
    }

    // print_board_visited(&visited_board);
    // print_energized(&visited_board);

    let num_energized = calc_energized(&visited_board);
    num_energized
}

pub fn main() {
    use rayon::prelude::*;

    println!("PART 2 ------------");
    let input = read_text("..\\Data\\input_1.txt".to_string());

    let layout: Vec<Vec<char>> = parse_input(input);
    let all_start_pos = get_start_positions(&layout);

    // println!("{:?}", all_start_pos);

    let max_energy: i32 = all_start_pos
        .par_iter()
        .map(|start_pos| solve_and_return_energy_score(&layout, start_pos.clone()))
        .max()
        .unwrap();

    println!("max_energy = {max_energy}");
}
