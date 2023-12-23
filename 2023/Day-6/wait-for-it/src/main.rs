mod part_1 {
    use crate::{count_winning_combinations, parse_input};

    fn calculate_all_win_possibilities() -> u64 {
        let mut distances: Vec<u64> = vec![];
        parse_input().iter().for_each(|(time, dist_to_beat)|{
            distances.push(count_winning_combinations((*time, *dist_to_beat)));
        });
        distances.iter().product()
    }

    pub fn solve() {
        println!("Product is for part1: {}", calculate_all_win_possibilities());
    }
}

mod part_2 {
    use crate::{parse_input, count_winning_combinations};

    pub fn solve() {
        let mut time_str: String = String::new();
        let mut dist_str: String = String::new();
        parse_input().iter().for_each(|(time, dist_to_beat)| {
            time_str += &time.to_string();
            dist_str += &dist_to_beat.to_string();
        });
        let time_and_dist: (u64, u64) = (time_str.parse::<u64>().unwrap(), dist_str.parse::<u64>().unwrap());
        println!("Product for part2: {}", count_winning_combinations(time_and_dist));
    }
}


fn count_winning_combinations((time, dist_to_beat): (u64, u64)) -> u64 {
    let initial_speed = 0;
    let mut count = 0;
    for i in 0..time + 1 {
        let distance = (initial_speed + i) * (time - i);
        if distance > dist_to_beat {
            count += 1;
        }
    }
    count
}



fn read_input() -> Vec<String> {
    std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>()
}

fn parse_input() -> Vec<(u64, u64)> {
    let input = read_input();
    assert_eq!(input.len(), 2);   // only two rows(time & distance)
    
    let mut td_vec: Vec<(u64, u64)> = Vec::new();
    let first_row = input.first().unwrap();
    let time_vec = 
        first_row.split("Time: ")
        .last()
        .unwrap()
        .split(" ")
        .filter(|el| !el.is_empty())
        .map(|el| el.trim().parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let second_row = input.last().unwrap();
    let distance_vec = 
        second_row.split("Distance: ")
        .last()
        .unwrap()
        .split(" ")
        .filter(|el| !el.is_empty())
        .map(|el| {el.trim().parse::<u64>().unwrap()})
        .collect::<Vec<u64>>();

    assert_eq!(distance_vec.len(), time_vec.len());

    for idx in 0..distance_vec.len() {
        td_vec.push((time_vec[idx], distance_vec[idx]));
    }
    
    td_vec
}

fn main() {
    part_1::solve();
    part_2::solve();
}
