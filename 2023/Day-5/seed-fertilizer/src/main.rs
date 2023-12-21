use std::collections::HashMap;

mod part_1 {
    use crate::parse_seeds_and_maps;

    pub fn solve() {
        let seed_fertilizers = parse_seeds_and_maps();
        seed_fertilizers.get_location_for_seed(79);
        println!("{:?}", seed_fertilizers);
    }
}

mod part_2 {

}


fn parse_input() -> Vec<String> {
    std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.trim().to_string())
        .collect::<Vec<String>>()
}

fn parse_seeds_and_maps() -> SeedFertilizer {
    let mut input = parse_input();
    let mut maps: Vec<Vec<Range>> = vec![];
    let seeds = input.first().unwrap().split(":").last()
            .unwrap()
            .split(" ")
            .filter(|num_str| !num_str.is_empty())
            .map(|num|
                num.trim().parse::<u64>().unwrap()
            ).collect::<Vec<u64>>();
    input.remove(0);

    let mut current_map_name: String = String::new();
    let mut current_range_vec = Vec::new();
    input
        .iter()
        .for_each(|line| {
            if line.contains(" map:") {
                if !current_range_vec.is_empty() {
                    maps.push(current_range_vec.clone());
                    current_range_vec.clear();
                }
            }
            else {
                let range_vec = line.split(" ")
                    .filter(|num_str| !num_str.is_empty())
                    .map(|num_str| num_str.trim().parse::<u64>().unwrap())
                    .collect::<Vec<u64>>();
                current_range_vec.push(Range::new(range_vec[0], range_vec[1], range_vec[2]));
            }
        });
    SeedFertilizer::new(seeds, maps)
}

#[derive(Debug)] #[derive(Clone)]
struct Range {
    destination: u64,
    source: u64, 
    length: u64
}

impl Range {
    pub fn new(destination: u64, source: u64, length: u64) -> Self {
        Self {
            destination, source, length
        }
    }

    pub fn mapping_for_number(&self, num: u64) -> Option<u64> {
        if self.source + self.length >= num {
            let needed_length = self.length + self.source - num;
            return Some(self.destination + needed_length);
        }
        None
    }
}

#[derive(Debug)] #[derive(Clone)]
struct SeedFertilizer
{
    seeds: Vec<u64>,
    maps: Vec<Vec<Range>>
}

impl SeedFertilizer {
    pub fn new(seeds: Vec<u64>, maps: Vec<Vec<Range>>) -> Self {
        Self {
            seeds,
            maps
        }
    }

    fn get_location_for_seed(&self, seed: u64) -> Option<u64> {
        let first_vec = self.maps.first().unwrap();
        let value = first_vec.iter()
            .map(|range| {
                range.mapping_for_number(seed)
            }).collect::<Vec<Option<u64>>>();
        println!("{:?}", value.iter().min());
        Some(1)
    }
}


fn main() {
    part_1::solve();
}
