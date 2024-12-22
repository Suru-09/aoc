use std::collections::{HashMap, VecDeque};

const INPUT: &str = "input.txt";
const EXAMPLE: &str = "example.txt";

pub fn solve_part_1() {
    let secret_numbers = parse_lines(INPUT);
    let result = secret_numbers.into_iter().fold(0isize, |acc, mut num| {
        simulate_nth_num(&mut num, 2000);
        acc + num
    });

    println!("Result for part1: {}", result);
}

fn sequence_to_tuple(seq: &VecDeque<isize>) -> (isize, isize, isize, isize) {
    assert_eq!(seq.len(), 4);
    (seq[0], seq[1], seq[2], seq[3])
}

pub fn solve_part_2() {
    let secret_numbers = parse_lines(INPUT);
    let mut all_sequences = HashMap::new();
    for number in secret_numbers {
        let mut sequences = HashMap::new();
        let mut curr_sequence = VecDeque::new();

        let mut last_digit = (number % 10) as isize;
        let mut num = number as isize;

        for _ in 0..2000 {
            next_number(&mut num);

            let new_digit = num % 10;
            let price_change = new_digit - last_digit;
            last_digit = new_digit;

            if curr_sequence.len() < 4 {
              curr_sequence.push_back(price_change);
            } else {
              curr_sequence.pop_front();
              curr_sequence.push_back(price_change);

              let seq_tuple = sequence_to_tuple(&curr_sequence);
              if sequences.get(&seq_tuple).is_none() {
                  sequences.insert(seq_tuple, last_digit);
              }
            }
        }

        for (k, v) in sequences.iter() {
          match all_sequences.get_mut(k) {
              Some(bananas) => {
                  *bananas += *v;
              }
              None => {
                  all_sequences.insert(k.clone(), *v);
              }
          }
        }
    }

    let result = all_sequences
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(_k, v)| v);
    println!("Result for part2: {:?}", result);
}

fn simulate_nth_num(secret: &mut isize, n: usize) {
    for _ in 0..n {
        next_number(secret);
    }
}

fn next_number(secret: &mut isize) {
    *secret = mix_prune_number(*secret, *secret << 6);
    *secret = mix_prune_number(*secret, *secret >> 5);
    *secret = mix_prune_number(*secret, *secret << 11);
}

fn mix_prune_number(secret_number: isize, number: isize) -> isize {
    (number ^ secret_number) % 16777216
}

fn parse_lines(input: &str) -> Vec<isize> {
    utils::read_file_lines(input)
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| line.trim().parse::<isize>().unwrap())
        .collect::<Vec<isize>>()
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
