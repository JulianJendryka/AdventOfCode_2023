use std::fs;

fn read_text(file_path: String) -> String {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    return contents;
}

fn parse_input(input: String) -> Vec<Vec<i128>> {
    input
        .split("\r\n")
        .into_iter()
        .map(|line| {
            line.split(" ")
                .map(|num| num.parse::<i128>().unwrap())
                .into_iter()
                .collect()
        })
        .collect()
}

fn find_sequence_ending(sequence: &Vec<i128>) -> i128 {
    if sequence.iter().all(|num| *num == 0) {
        return 0;
    }

    let differences: Vec<i128> = sequence
        .iter()
        .zip(sequence.iter().skip(1))
        .map(|(current, next)| next - current)
        .collect();

    println!("differences {:?}", differences);
    return differences.iter().last().unwrap() + find_sequence_ending(&differences);
}

pub fn main() {
    println!("PART 1 ------------");
    let input = read_text("..\\Data\\input_1.txt".to_string());

    let readings = parse_input(input);

    let sum: i128 = readings
        .into_iter()
        .map(|sequence: Vec<i128>| {
            println!("\n");
            println!("differences {:?}", sequence);
            sequence.iter().last().unwrap() + find_sequence_ending(&sequence)
        })
        .sum();
    println!("sum {:?}", sum);
}
