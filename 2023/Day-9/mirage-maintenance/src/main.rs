mod part_1 {
    use crate::{read_input, calculate_next_for_line};

    pub fn solve() {
        let answer: i64 = read_input().iter()
            .map(|line| calculate_next_for_line(line))
            .collect::<Vec<_>>()
            .iter().sum();

        println!("Result for part1 is: {answer}");
    }
}

mod part_2 {
    use crate::{read_input, calculate_prev_for_line};

    pub fn solve() {
        let answer: i64 = read_input().iter()
            .map(|line| calculate_prev_for_line(line))
            .collect::<Vec<_>>()
            .iter().sum();

        println!("Result for part2 is: {answer}");
    }
}

fn read_input() -> Vec<String> {
    std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| line.trim().to_string())
        .collect()
}

fn parse_line(line: &str) -> Vec<i64> {
    line.split(" ")
        .collect::<Vec<&str>>()
        .iter()
        .map(|num| num.trim().parse::<i64>().unwrap())
        .collect()
}

fn extended_vec(line: &str) -> Vec<Vec<i64>> {
    let mut current_vec = parse_line(line);
    let mut total_vec = Vec::new();
    total_vec.push(current_vec.clone());

    loop {
        let diff_vec = construct_difference_vec(&current_vec);
        current_vec = diff_vec.clone();
        total_vec.push(diff_vec.clone());

        if diff_vec.iter().all(|val| *val == 0) {
            break
        }
    }
    total_vec
}

fn calculate_next_for_line(line: &str) -> i64 {
    let mut previous_value: i64 = 0;
    let mut searched_value: i64 = 0;
    for diff_vec in extended_vec(line).iter().rev() {
        searched_value =  diff_vec.last().unwrap() + previous_value;
        previous_value = searched_value;
    }
    searched_value
}

fn calculate_prev_for_line(line: &str) -> i64 {
    let mut previous_extrapolated: i64 = 0;
    let mut searched_value: i64 = 0;
    for diff_vec in extended_vec(line).iter().rev() {
        searched_value = diff_vec.first().unwrap() - previous_extrapolated;
        previous_extrapolated = searched_value;
    }
    searched_value
}

fn construct_difference_vec(initial_vec: &Vec<i64>) -> Vec<i64> {
    let mut difference_vec = vec![];
    let mut previous_el = initial_vec.first().unwrap().clone();
    for idx in 1..initial_vec.len() {
        difference_vec.push(initial_vec[idx] - previous_el);
        previous_el = initial_vec[idx].clone();
    }
    difference_vec
}

fn main() {
    part_1::solve();    // 1938800261
    part_2::solve();    // 1112
}
