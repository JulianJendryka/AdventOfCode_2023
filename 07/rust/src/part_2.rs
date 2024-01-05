use std::{collections::HashMap, fs};

fn read_text(file_path: String) -> String {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    return contents;
}

#[derive(Debug)]
pub struct Hand {
    _cards: String,
    bet: i32,
    key: u64,
}

impl Hand {
    pub fn create(line: &str) -> Hand {
        let mut s = line.split(" ");
        let hand = s.next().unwrap();
        let bet = s.next().unwrap();

        Hand {
            _cards: hand.to_string(),
            bet: bet.parse::<i32>().unwrap(),
            key: Hand::get_key(hand),
        }
    }

    fn get_key(cards: &str) -> u64 {
        let cardinality = Hand::get_cardinality(cards);
        let score_cardinality = Hand::get_score_from_cardinality(cardinality);
        let score_cards = Hand::get_score_from_cards(cards);

        let mut total_score = String::from(score_cardinality.to_string());
        total_score.push_str(score_cards.as_str());

        total_score.parse::<u64>().unwrap()
    }

    fn get_cardinality(hand: &str) -> Vec<i32> {
        let mut valid_cards: HashMap<char, i32> = HashMap::from([
            ('A', 0),
            ('K', 0),
            ('Q', 0),
            ('J', 0),
            ('T', 0),
            ('9', 0),
            ('8', 0),
            ('7', 0),
            ('6', 0),
            ('5', 0),
            ('4', 0),
            ('3', 0),
            ('2', 0),
            ('J', 0),
        ]);

        for c in hand.chars().into_iter() {
            let x = valid_cards.get(&c).unwrap().clone() + 1;
            valid_cards.insert(c, x);
        }
        let num_jokers = valid_cards.get(&'J').unwrap().clone();

        let mut cardinality: Vec<i32> = valid_cards
            .into_iter()
            .filter(|(_, count)| count.clone() > 0)
            .filter(|(c, _)| c.clone() != 'J')
            .map(|(_, count)| count)
            .collect();
        cardinality.sort();
        cardinality.reverse();

        if cardinality.len() == 0 {
            cardinality = vec![5];
        } else {
            cardinality[0] = cardinality.first().unwrap().clone() + num_jokers;
        }

        cardinality
    }

    fn get_score_from_cardinality(cardinality: Vec<i32>) -> i32 {
        let s: Vec<String> = cardinality.iter().map(|num| num.to_string()).collect();
        let name = s.join("");

        match name.as_str() {
            "5" => 7,
            "41" => 6,
            "32" => 5,
            "311" => 4,
            "221" => 3,
            "2111" => 2,
            "11111" => 1,
            _ => panic!("INVALID COMBINATION = {name}"),
        }
    }

    fn get_score_from_cards(cards: &str) -> String {
        let card_value: HashMap<char, &str> = HashMap::from([
            ('A', "14"),
            ('K', "13"),
            ('Q', "12"),
            ('J', "11"),
            ('T', "10"),
            ('9', "09"),
            ('8', "08"),
            ('7', "07"),
            ('6', "06"),
            ('5', "05"),
            ('4', "04"),
            ('3', "03"),
            ('2', "02"),
            ('J', "01"),
        ]);

        let values_strings: Vec<&str> = cards
            .chars()
            .into_iter()
            .map(|c| {
                let x = card_value.get(&c).unwrap();
                let y = *x;
                y
            })
            .collect();

        let values_string = values_strings.join("");
        values_string
    }
}

fn prepare_input(input: &String) -> Vec<Hand> {
    input
        .split("\r\n")
        .into_iter()
        .map(|line| Hand::create(line))
        .collect()
}

fn sort_hands(cards_to_bet: Vec<Hand>) -> Vec<Hand> {
    let mut sorted_cards_to_bet: Vec<Hand> = cards_to_bet.into_iter().collect();
    sorted_cards_to_bet.sort_by_key(|x| x.key);
    sorted_cards_to_bet
}

pub fn main() {
    println!("PART 2 ------------");
    let input = read_text("..\\Data\\input_1.txt".to_string());

    let cards_to_bet = prepare_input(&input);
    let sorted_cards_to_bet = sort_hands(cards_to_bet);

    let sum: i32 = sorted_cards_to_bet
        .iter()
        .enumerate()
        .map(|(index, hand)| (1 + (index as i32)) * hand.bet)
        .sum();
    println!("{sum}");
}
