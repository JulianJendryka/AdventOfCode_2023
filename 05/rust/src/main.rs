use std::{collections::HashMap, fs};

fn read_text(file_path: String) -> String {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    return contents;
}

fn get_cleaned_input(input: &String) -> (Vec<u64>, HashMap<&str, Vec<Vec<u64>>>) {
    let mut seeds = vec![];
    let mut maps = HashMap::new();
    for (index, line) in input.split("\r\n\r\n").enumerate() {
        if index == 0 {
            seeds = line
                .split(": ")
                .into_iter()
                .last()
                .unwrap()
                .split(" ")
                .map(|x| x.parse::<u64>().unwrap())
                .collect();
        } else {
            let mut splitted_line = line.split(":\r\n");
            let name = splitted_line.next().unwrap();

            let nums = splitted_line.next().unwrap();
            let rows: Vec<Vec<u64>> = nums
                .split("\r\n")
                .into_iter()
                .map(|x| {
                    x.split(" ")
                        .into_iter()
                        .map(|i| i.parse::<u64>().unwrap())
                        .collect()
                })
                .collect();
            maps.insert(name, rows);
        }
    }

    (seeds, maps)
}

pub struct SourceMapLine {
    destination_range_start: u64,
    _range_length: u64,
    source_range_start: u64,
    source_range_end: u64,
}

impl SourceMapLine {
    pub fn create(line: &Vec<u64>) -> SourceMapLine {
        let map = SourceMapLine {
            destination_range_start: line.get(0).unwrap().clone(),
            _range_length: line.get(2).unwrap().clone(),

            source_range_start: line.get(1).unwrap().clone(),
            source_range_end: line.get(1).unwrap() + line.get(2).unwrap(),
        };
        map
    }

    pub fn solve(&self, num: u64) -> (bool, u64) {
        if num < self.source_range_start || num >= self.source_range_end {
            return (false, 1);
        }

        let mapped_value = num - self.source_range_start + self.destination_range_start;
        (true, mapped_value)
    }
}

pub struct SourceMap {
    mappings: Vec<SourceMapLine>,
}

impl SourceMap {
    pub fn create(line: &Vec<Vec<u64>>) -> SourceMap {
        SourceMap {
            mappings: line.iter().map(|l| SourceMapLine::create(l)).collect(),
        }
    }

    pub fn get_mapping(&self, query_num: u64) -> u64 {
        for map in self.mappings.iter() {
            let (is_inside, mapped_val) = map.solve(query_num).clone();
            if is_inside {
                return mapped_val;
            }
        }
        query_num
    }
}

pub struct GardenerBook {
    seed_to_soil: SourceMap,
    soil_to_fertilizer: SourceMap,
    fertilizer_to_water: SourceMap,
    water_to_light: SourceMap,
    light_to_temperature: SourceMap,
    temperature_to_humidity: SourceMap,
    humidity_to_location: SourceMap,
}

impl GardenerBook {
    pub fn create(input: HashMap<&str, Vec<Vec<u64>>>) -> GardenerBook {
        GardenerBook {
            seed_to_soil: SourceMap::create(input.get("seed-to-soil map").unwrap()),
            soil_to_fertilizer: SourceMap::create(input.get("soil-to-fertilizer map").unwrap()),
            fertilizer_to_water: SourceMap::create(input.get("fertilizer-to-water map").unwrap()),
            water_to_light: SourceMap::create(input.get("water-to-light map").unwrap()),
            light_to_temperature: SourceMap::create(input.get("light-to-temperature map").unwrap()),
            temperature_to_humidity: SourceMap::create(
                input.get("temperature-to-humidity map").unwrap(),
            ),
            humidity_to_location: SourceMap::create(input.get("humidity-to-location map").unwrap()),
        }
    }

    pub fn get_location_of_seed(&self, seed: u64) -> u64 {
        let soil = self.seed_to_soil.get_mapping(seed);
        let fertilizer = self.soil_to_fertilizer.get_mapping(soil);
        let water = self.fertilizer_to_water.get_mapping(fertilizer);
        let light = self.water_to_light.get_mapping(water);
        let temperature = self.light_to_temperature.get_mapping(light);
        let humidity = self.temperature_to_humidity.get_mapping(temperature);

        let location = self.humidity_to_location.get_mapping(humidity);
        location
    }
}

// fn test_1() {
//     let input = read_text("..\\Data\\test_input_1.txt".to_string());
//     let (seeds, maps) = get_cleaned_input(&input);

//     let mappa = SourceMap::create(&maps.get("seed-to-soil map").unwrap());
//     assert!(mappa.get_mapping(79) == 81);
//     assert!(mappa.get_mapping(14) == 14);
//     assert!(mappa.get_mapping(55) == 57);
//     assert!(mappa.get_mapping(13) == 13);

//     let book = GardenerBook::create(maps);
//     assert!(book.get_location_of_seed(79) == 82);
//     assert!(book.get_location_of_seed(14) == 43);
//     assert!(book.get_location_of_seed(55) == 86);
//     assert!(book.get_location_of_seed(13) == 35);

//     let min_loc = seeds
//         .iter()
//         .map(|seed| book.get_location_of_seed(seed.clone()))
//         .min()
//         .unwrap();

//     println!("min_loc: \t{min_loc}");
// }

fn run_1() {
    let input = read_text("..\\Data\\input_1.txt".to_string());
    let (seeds, maps) = get_cleaned_input(&input);

    let book = GardenerBook::create(maps);

    let min_loc = seeds
        .iter()
        .map(|seed| book.get_location_of_seed(seed.clone()))
        .min()
        .unwrap();

    println!("min_loc: \t{min_loc}");
}

// fn test_2() {
//     use indicatif::ProgressIterator;
//     use rayon::prelude::*;

//     let input = read_text("..\\Data\\test_input_1.txt".to_string());
//     let (seeds, maps) = get_cleaned_input(&input);

//     let book = GardenerBook::create(maps);

//     let chunks: Vec<&[u64]> = seeds.chunks(2).collect();
//     let min_loc = chunks
//         .iter()
//         .progress()
//         .map(|c| {
//             let all_seeds: Vec<u64> = (c[0]..(c[0] + c[1])).into_iter().collect();
//             let min = all_seeds
//                 .par_iter()
//                 .map(|seed| book.get_location_of_seed(seed.clone()))
//                 .min()
//                 .unwrap();
//             min
//         })
//         .min()
//         .unwrap();

//     println!("min_loc: \t{min_loc}");
// }

fn run_2() {
    use indicatif::ProgressIterator;
    use rayon::prelude::*;

    let input = read_text("..\\Data\\input_1.txt".to_string());
    let (seeds, maps) = get_cleaned_input(&input);

    let book = GardenerBook::create(maps);

    let chunks: Vec<&[u64]> = seeds.chunks(2).collect();
    let min_loc = chunks
        .iter()
        .progress()
        .map(|c| {
            let all_seeds: Vec<u64> = (c[0]..(c[0] + c[1])).into_iter().collect();
            let min = all_seeds
                .par_iter()
                .map(|seed| book.get_location_of_seed(seed.clone()))
                .min()
                .unwrap();
            min
        })
        .min()
        .unwrap();

    println!("min_loc: \t{min_loc}");
}

fn main() {
    // test_1();
    run_1();
    // test_2();
    run_2();
}
