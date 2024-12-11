use std::collections::HashMap;
use std::time::Instant;

const INPUT: &str = "input.txt";
const EXAMPLE: &str = "example.txt";


pub fn solve_part_1() {
    let pebbles_str = utils::read_file(INPUT);
    let pebbles = parse_pebbles(&pebbles_str);

    let now = Instant::now();
    let result;
    {
        result = pebble_expansion(&pebbles, 25);
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    println!("Result for part1: {}", result);
}

pub fn solve_part_2() {
    let pebbles_str = utils::read_file(INPUT);
    let pebbles = parse_pebbles(&pebbles_str);

    let now = Instant::now();
    let result;
    {
        result = pebble_expansion(&pebbles, 200);
    }
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
        let mut evaluated: Vec<Vec<(u128, u128)>> = vec![];
        let mut removed_pebbles = vec![];
        for (pebble, p_count) in pebbles_count.iter() {
            evaluated.push(
                evaluate_pebble(*pebble)
                    .iter()
                    .map(|pebble| (*pebble, *p_count))
                    .collect(),
            );
            removed_pebbles.push((*pebble, *p_count));
        }

        removed_pebbles
            .iter()
            .for_each(|(pebble, p_count)| match pebbles_count.get(pebble) {
                Some(val) => {
                    if *val > *p_count {
                        *pebbles_count.get_mut(pebble).unwrap() = *val - *p_count;
                    } else {
                        pebbles_count.remove(pebble);
                    }
                }
                None => {}
            });

        evaluated.iter().flatten().for_each(|(pebble, p_count)| {
            match pebbles_count.get_mut(pebble) {
                Some(val) => *val += p_count,
                None => {
                    pebbles_count.insert(*pebble, *p_count);
                }
            }
        });
    }

    pebbles_count
        .iter()
        .fold(0u128, |sum, (_, p_count)| sum + p_count)
}

fn evaluate_pebble(pebble: u128) -> Vec<u128> {
    let mut evaluated = vec![];
    if pebble == 0 {
        evaluated.push(1);
    } else {
        let pdigits_count = digits_count(pebble);
        if pdigits_count % 2 == 0 {
            let second_half: u128 = format!("{}", pebble)[(pdigits_count / 2) as usize..].parse().unwrap();
            let first_half: u128 = format!("{}", pebble)[0..(pdigits_count / 2) as usize].parse().unwrap();
            evaluated.push(first_half);
            evaluated.push(second_half);
        } else {
            evaluated.push(pebble * 2024);
        }
    }
    evaluated
}

fn digits_count(num: u128) -> u128 {
    num.checked_ilog10().unwrap_or(0) as u128 + 1 as u128
}

fn parse_pebbles(input: &str) -> Vec<u128> {
    input
        .trim()
        .split(" ")
        .map(|x| x.parse::<u128>().unwrap())
        .collect::<Vec<u128>>()
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
