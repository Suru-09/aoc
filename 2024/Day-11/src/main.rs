use std::collections::HashMap;
use std::time::Instant;

const INPUT: &str = "input.txt";
const EXAMPLE: &str = "example.txt";


pub fn solve_part_1() {
    let pebbles_str = utils::read_file(INPUT);
    let pebbles = parse_pebbles(&pebbles_str);

    let now = Instant::now();
    let result = pebble_expansion(&pebbles, 25);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    println!("Result for part1: {}", result);
}

pub fn solve_part_2() {
    let pebbles_str = utils::read_file(INPUT);
    let pebbles = parse_pebbles(&pebbles_str);

    let now = Instant::now();
    let result = pebble_expansion(&pebbles, 75);
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    println!("Result for part2: {}", result);
}

fn pebble_expansion(pebbles: &Vec<u128>, blinks: u128) -> u128 {
    let mut pebbles_count: HashMap<u128, u128> = HashMap::new();
    pebbles
        .iter()
        .for_each(|pebble| match pebbles_count.get_mut(pebble) {
            Some(val) => *val += 1,
            None => {
                pebbles_count.insert(*pebble, 1);
            }
        });

    for _ in 0..blinks {
        let mut pebbles_temp: HashMap<u128, u128> = HashMap::new();
        for (pebble, p_counter) in pebbles_count.iter() {
            evaluate_pebble(*pebble).into_iter().for_each(|pebl| {
                match pebbles_temp.get_mut(&pebl) {
                    Some(val) => *val += *p_counter,
                    None => {
                        pebbles_temp.insert(pebl, *p_counter);
                    }
                }
            });
        }
        pebbles_count = pebbles_temp;
    }

    pebbles_count
        .iter()
        .fold(0u128, |sum, (_, p_count)| sum + p_count)
}

fn evaluate_pebble(pebble: u128) -> Vec<u128> {
    if pebble == 0 {
        vec![1]
    } else {
        let pdigits_count: u128 = digits_count(pebble);
        if pdigits_count % 2 == 0 {
            let poweeer: u128 = 10u32.pow(pdigits_count as u32 / 2) as u128;
            let first_half: u128 = pebble / poweeer;
            let second_half: u128 = pebble % poweeer;
            vec![first_half, second_half]
        } else {
            vec![pebble * 2024]
        }
    }
}

fn digits_count(num: u128) -> u128 {
    num.checked_ilog10().unwrap_or(0) as u128 + 1
}

fn parse_pebbles(input: &str) -> Vec<u128> {
    input
        .trim()
        .split(" ")
        .map(|x| x.parse::<u128>().unwrap())
        .collect::<Vec<_>>()
}

mod utils {
    pub fn read_file(path: &str) -> String {
        std::fs::read_to_string(path).unwrap()
    }

    pub fn read_file_lines(path: &str) -> Vec<String> {
        std::fs::read_to_string(path)
            .unwrap()
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<_>>()
    }
}

fn main() {
    solve_part_1();
    solve_part_2();
}
