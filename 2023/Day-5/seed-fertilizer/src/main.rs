mod part_1 {
    use crate::parse_seeds_and_maps;

    pub fn solve() {
        let mut results: Vec<u64> = vec![];
        let seed_fertilizers = parse_seeds_and_maps();

        for seed_id in 0..seed_fertilizers.seeds.len() {
            let mut value = seed_fertilizers.seeds[seed_id];
            for map_id in 0..seed_fertilizers.maps.len() {
                value = match seed_fertilizers.get_location_for_seed(value, map_id) {
                    Some(location) => location,
                    None => value
                };
                if map_id == 6 {
                    results.push(value);
                    break;
                }
            }
        }
        let result = results.iter().min().unwrap_or(&0);
        println!("Result is : {}", result);
    }
}

mod part_2 {
    use crate::parse_seeds_and_maps;

    pub fn solve() {
        let mut results: Vec<u64> = vec![];
        let seed_fertilizers = parse_seeds_and_maps();

        let seeds = get_seeds_given_ranges(seed_fertilizers.seeds.clone());
        for seed_id in 0..seeds.len() {
            let mut value = seeds[seed_id];
            for map_id in 0..seed_fertilizers.maps.len() {
                value = match seed_fertilizers.get_location_for_seed(value, map_id) {
                    Some(location) => location,
                    None => value
                };
                if map_id == 6 {
                    results.push(value);
                    break;
                }
            }
        }
        let result = results.iter().min().unwrap_or(&0);
        println!("Result is : {}", result);
    }

    fn get_seeds_given_ranges(seeds_ranges: Vec<u64>) -> Vec<u64> {
        let mut seeds: Vec<u64> = vec![];
        for idx in (0..seeds_ranges.len()).step_by(2) {
            let range_start = seeds_ranges[idx];
            let range_length = seeds_ranges[idx + 1];
            for offset in 0..range_length {
                seeds.push(range_start + offset);
            }
        }
        seeds
    }
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
        if !current_range_vec.is_empty() {
            maps.push(current_range_vec.clone());
            current_range_vec.clear();
        }
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
            for i in 0..self.length {
                if self.source + i == num {
                    return Some(i + self.destination)
                }
            }
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

    fn get_location_for_seed(&self, seed: u64, map_number: usize) -> Option<u64> {
        let map = self.maps.get(map_number).unwrap();
        for range in map {
            if range.mapping_for_number(seed).is_some() {
                return range.mapping_for_number(seed);
            }
        }
        None
    }


}


fn main() {
    part_1::solve(); // 174137457
    part_2::solve(); // 1493866
}
