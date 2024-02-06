use std::collections::VecDeque;
use std::cmp;

mod part_1 {
    use crate::{read_input, galaxies_pairs, shortest_distance, is_row_col_empty};

    pub fn solve() {
        let mut sum = 0;
        let expanded_space = read_input();
        let galaxies_pairs = galaxies_pairs(&expanded_space.clone());
        let row_and_col_empty = is_row_col_empty(&expanded_space);
        println!("Galaxy pairs length: {}", galaxies_pairs.len());
        galaxies_pairs.iter().for_each(|(gal_1, gal_2)| {
            sum += shortest_distance(gal_1.clone(), gal_2.clone(), &row_and_col_empty.0, &row_and_col_empty.1, 1);
        });

        println!("Result for part_1: {}", sum);
    }
}

mod part_2 {
    use crate::{read_input, galaxies_pairs, shortest_distance, is_row_col_empty};

    pub fn solve() {
        let mut sum = 0;
        let expanded_space = read_input();
        let galaxies_pairs = galaxies_pairs(&expanded_space.clone());
        let row_and_col_empty = is_row_col_empty(&expanded_space);
        println!("Galaxy pairs length: {}", galaxies_pairs.len());
        galaxies_pairs.iter().for_each(|(gal_1, gal_2)| {
            sum += shortest_distance(gal_1.clone(), gal_2.clone(), &row_and_col_empty.0, &row_and_col_empty.1, 999999);
        });

        println!("Result for part_2: {}", sum);
    }
}

#[derive(Clone, Copy, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

fn galaxies_pairs(expanded_space: &Vec<String>) -> Vec<(Point, Point)> {
    let mut galaxies_positions = Vec::new();
    let mut galaxies_pairs = Vec::new();
    for (row_idx, row) in expanded_space.iter().enumerate() {
        for (col_idx, col_val) in row.chars().enumerate() {
            if col_val == '#' {
                galaxies_positions.push(Point{x: row_idx, y: col_idx});
            }
        }
    }

    for i in 0..galaxies_positions.len() {
        for j in i + 1..galaxies_positions.len() {
            galaxies_pairs.push((galaxies_positions[i], galaxies_positions[j]));
        }
    }
    galaxies_pairs
}

fn is_row_col_empty(expanded_space: &Vec<String>) -> (Vec<bool>, Vec<bool>) {
    let mut is_row_empty = vec![true; expanded_space.len()];
    let mut is_col_empty = vec![true; expanded_space[0].len()];
    for (row_idx, row) in expanded_space.iter().enumerate() {
        for (col_idx, col_val) in row.chars().enumerate() {
            if col_val == '#' {
                is_row_empty[row_idx] = false;
                is_col_empty[col_idx] = false;
            }
        }
    }

    (is_row_empty, is_col_empty)
}

fn shortest_distance(start: Point, end: Point, is_row_empty: &Vec<bool>, is_col_empty: &Vec<bool>, expansion_coef: usize) -> usize {
    let mut dist = ((start.x as isize - end.x as isize).abs() + (start.y as isize - end.y as isize).abs()) as usize;

    for i in cmp::min(start.x, end.x) + 1..cmp::max(start.x, end.x) {
        if is_row_empty[i] == true {
            dist += expansion_coef;
        }
    }

    for i in cmp::min(start.y, end.y) + 1..cmp::max(start.y, end.y) {
        if is_col_empty[i] == true {
            dist += expansion_coef;
        }
    }

    dist
}

fn read_input() -> Vec<String> {
    std::fs::read_to_string("input.txt")
        .unwrap().lines()
        .map(|line|line.to_string())
        .collect::<Vec<String>>()
}

fn main() {
    part_1::solve();    // 10313550
    part_2::solve();    // 611998089572
}
