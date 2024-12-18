use std::collections::{HashMap, VecDeque};

const INPUT: &str = "input.txt";
const EXAMPLE: &str = "example.txt";

const DIRS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

const SIZE: usize = 71;
const STOP: usize = 1024;

pub fn solve_part_1() {
    let obstacles_str = utils::read_file(INPUT);
    let matrix = apply_n_obstacles(&obstacles_str, SIZE, STOP);
    let result = bfs(&matrix, SIZE);
    println!("Result for part1: {:?}", result);
}

pub fn solve_part_2() {
    let obstacles_str = utils::read_file(INPUT);
    let obstacles = parse_obstacles(&obstacles_str);
    let mut left = STOP;
    let mut right = obstacles.len();
    let mut result = None;

    while left <= right {
        let middle = left + (right - left) / 2;
        let matrix = apply_n_obstacles(&obstacles_str, SIZE, middle);
        match bfs(&matrix, SIZE) {
            Some(_) => {
                result = Some(obstacles[middle]);
                left = middle + 1;
            }
            None => right = middle - 1,
        }
    }

    println!("Result for part2: {:?}", result);
}

fn bfs(matrix: &Vec<Vec<char>>, size: usize) -> Option<usize> {
    let mut queue = VecDeque::new();
    let mut visited = HashMap::new();
    let (start, end) = ((0, 0), (size - 1, size - 1));
    queue.push_back((start, 0));
    visited.insert(start, 0);

    while let Some(((i, j), cost)) = queue.pop_front() {
        if cost > *visited.get(&(i, j)).unwrap() {
            continue;
        }

        if (i, j) == end {
            break;
        }

        for d in DIRS {
            let (valid, (new_x, new_y)) = is_valid(matrix, size, (i, j), d);
            if valid && visited.get(&(new_x, new_y)).is_none() {
                match visited.get_mut(&(new_x, new_y)) {
                    Some(vis_val) => *vis_val = cost + 1,
                    None => {
                        visited.insert((new_x, new_y), cost + 1);
                    }
                }
                queue.push_back(((new_x, new_y), cost + 1));
            }
        }
    }

    match visited.get(&end) {
        Some(val) => Some(*val),
        None => None,
    }
}

fn is_valid(
    matrix: &Vec<Vec<char>>,
    size: usize,
    old: (usize, usize),
    dir: (isize, isize),
) -> (bool, (usize, usize)) {
    let size = size as isize;
    let (new_x, new_y) = (old.0 as isize + dir.0, old.1 as isize + dir.1);
    if new_x < 0 || new_y < 0 || new_x >= size || new_y >= size {
        return (false, (0, 0));
    }

    if matrix[new_x as usize][new_y as usize] == '#' {
        return (false, (0, 0));
    }

    (true, (new_x as usize, new_y as usize))
}

fn parse_obstacles(input: &str) -> Vec<(usize, usize)> {
    input
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|str| {
            let tuple = str.split(",").collect::<Vec<&str>>();
            (
                tuple[0].trim().parse::<usize>().unwrap(),
                tuple[1].trim().parse::<usize>().unwrap(),
            )
        })
        .collect::<Vec<(usize, usize)>>()
}

fn apply_n_obstacles(input: &str, size: usize, stop: usize) -> Vec<Vec<char>> {
    let obstacles = parse_obstacles(input);

    let mut matrix = vec![vec!['.'; size]; size];
    for (idx, (ox, oy)) in obstacles.iter().enumerate() {
        if idx == stop {
            break;
        }
        matrix[*oy][*ox] = '#';
    }

    matrix
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
