use std::collections::HashMap;

const INPUT: &str = "input.txt";
const EXAMPLE: &str = "example.txt";

pub fn solve_part_1() {
    let input_str = &utils::read_file(INPUT);
    let (patterns, designs) = parse_towels(input_str);
    let mut cache = HashMap::new();
    let result = designs
        .iter()
        .filter(|design| dfs(&patterns, design, 0, &mut cache) >= 1)
        .count();
    println!("Result for part1: {}", result);
}

pub fn solve_part_2() {
    let input_str = &utils::read_file(INPUT);
    let (patterns, designs) = parse_towels(input_str);
    let mut cache = HashMap::new();
    let result = designs.iter().fold(0usize, |acc, design| {
        acc + dfs(&patterns, design, 0, &mut cache)
    });
    println!("Result for part2: {}", result);
}

fn dfs<'a>(
    patterns: &Vec<&str>,
    t_design: &'a str,
    idx: usize,
    cache: &mut HashMap<&'a str, usize>,
) -> usize {
    if let Some(val) = cache.get(&t_design[idx..]) {
        return *val;
    }

    let mut counter = 0;
    for p in patterns {
        if p.len() + idx > t_design.len() || t_design[idx..idx + p.len()] != **p {
            continue;
        }

        if t_design[..idx + p.len()] == *t_design {
            counter += 1;
        }

        counter += dfs(patterns, t_design, idx + p.len(), cache);
    }

    cache.insert(&t_design[idx..], counter);
    counter
}

fn parse_towels(input: &str) -> (Vec<&str>, Vec<&str>) {
    let lines = input
        .lines()
        .filter(|line| !line.is_empty())
        .collect::<Vec<&str>>();
    let patterns = lines[0]
        .split(", ")
        .filter(|p| !p.is_empty())
        .collect::<Vec<&str>>();
    (patterns, lines[1..].to_vec())
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
