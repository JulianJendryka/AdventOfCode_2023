mod hash;
mod lense;
mod lense_box;

use lense::Lense;
use lense_box::Box;
use std::fs;

fn read_text(file_path: String) -> String {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    return contents;
}

fn parse_input(input: &String) -> Vec<Lense> {
    input
        .split(",")
        .into_iter()
        .map(|line| Lense::create(line.to_string()))
        .collect()
}

fn print_all_boxes(all_boxes: &Vec<Box>) {
    for (i, b) in all_boxes.iter().enumerate() {
        if b.lenses.is_empty() {
            continue;
        }

        print!("Box {i}:");
        for lense in b.lenses.iter() {
            lense.print()
        }
        print!("\n");
    }
}

fn calculate_total_focusing_power(all_boxes: Vec<Box>) -> u128 {
    all_boxes
        .into_iter()
        .enumerate()
        .map(|(box_index, b)| {
            b.lenses
                .into_iter()
                .enumerate()
                .map(|(lense_index, lense)| {
                    (box_index + 1) as u128 * (lense_index + 1) as u128 * lense.focal_length as u128
                })
                .sum::<u128>()
        })
        .sum()
}

pub fn main() {
    println!("PART 2 ------------");

    let input = read_text("..\\Data\\input_1.txt".to_string());
    let all_lenses = parse_input(&input);

    // println!("{:?}", all_lenses);

    let mut all_boxes: Vec<Box> = (0..256).into_iter().map(|_| Box::create_empty()).collect();

    for lense in all_lenses {
        let box_index = lense.hash as usize;
        all_boxes[box_index].handle_lense(lense);
    }

    print_all_boxes(&all_boxes);

    let total_focusing_power = calculate_total_focusing_power(all_boxes);
    println!("total_focusing_power = {total_focusing_power}");
}
