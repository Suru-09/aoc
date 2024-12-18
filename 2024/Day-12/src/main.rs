use std::collections::{HashMap, HashSet};

const INPUT: &str = "input.txt";
const EXAMPLE: &str = "example.txt";

const DIRS: [(i32, i32); 4] = [(0, -1), (0, 1), (1, 0), (-1, 0)];

pub fn solve_part_1() {
    let garden_str = utils::read_file(INPUT);
    let garden = parse_garden(&garden_str);
    //println!("Garden: {:?}", garden);

    let (width, height) = (garden[0].len(), garden.len());

    let mut visited = vec![vec![false; width]; height];
    let mut garden_groups = vec![];

    for i in 0..garden.len() {
        for (j, val) in garden[i].iter().enumerate() {
            if !visited[i][j] {
                let mut group = vec![(i, j)];
                find_group(&garden, &mut visited, &mut group, (i, j), *val);
                garden_groups.push(group);
            }
        }
    }

    let result = garden_groups.iter().fold(0usize, |sum, group| {
        sum + group.iter().fold(0usize, |perimeter, element| {
            perimeter + (4 - find_neighbors(*element, &garden).len())
        }) * group.len()
    });
    println!("Result for part1: {}", result);
}

pub fn solve_part_2() {
    let garden_str = utils::read_file(INPUT);
    let garden = parse_garden(&garden_str);
    //println!("Garden: {:?}", garden);

    let (width, height) = (garden[0].len(), garden.len());

    let mut visited = vec![vec![false; width]; height];
    let mut garden_groups = vec![];

    for i in 0..garden.len() {
        for (j, val) in garden[i].iter().enumerate() {
            if !visited[i][j] {
                let mut group = vec![(i, j)];
                find_group(&garden, &mut visited, &mut group, (i, j), *val);
                garden_groups.push(group);
            }
        }
    }

    let result = garden_groups.iter().fold(0usize, |sum, group| {
        let corners = count_corners(group);
        // println!(
        //     "Area * corners = {} * {} = {}",
        //     group.len(),
        //     corners,
        //     corners * group.len()
        // );
        sum + corners * group.len()
    });
    println!("Result for part2: {}", result);
}

fn count_corners(group: &Vec<(usize, usize)>) -> usize {
    let mut corners = 0;

    for dir in DIRS {
        let mut sides = HashSet::new();
        for pos in group {
            let (new_x, new_y): (i32, i32) = (pos.0 as i32 + dir.0, pos.1 as i32 + dir.1);
            if !group.contains(&(new_x as usize, new_y as usize)) {
                sides.insert((new_x, new_y));
            }
        }
        let mut remove = HashSet::new();
        for side in sides.iter() {
            let (mut new_x, mut new_y): (i32, i32) = (side.0 as i32 + dir.1, side.1 as i32 + dir.0);
            if sides.contains(&(new_x, new_y)) {
                remove.insert((new_x, new_y));
                // new_x += dir.1;
                // new_y += dir.0;
            }
        }
        corners += sides.len() - remove.len();
    }

    corners
}

fn find_neighbors(element: (usize, usize), garden: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let (width, height) = (garden[0].len(), garden.len());
    let (x, y) = element;

    let mut neighbors = vec![];

    for dir in DIRS {
        if !is_out_of_bounds((width, height), element, dir) {
            let (new_x, new_y) = ((x as i32 + dir.0) as usize, (y as i32 + dir.1) as usize);
            if garden[new_x][new_y] == garden[x][y] {
                neighbors.push((new_x, new_y));
            }
        }
    }
    neighbors
}

fn is_out_of_bounds(wh: (usize, usize), old: (usize, usize), dir: (i32, i32)) -> bool {
    let (width, height) = wh;
    let (new_x, new_y) = (old.0 as i32 + dir.0, old.1 as i32 + dir.1);
    if new_x < 0 || new_x as usize >= height || new_y < 0 || new_y as usize >= width {
        true
    } else {
        false
    }
}

fn find_group(
    garden: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    garden_group: &mut Vec<(usize, usize)>,
    pos: (usize, usize),
    group_type: char,
) {
    let (x, y) = pos;
    visited[x][y] = true;

    let (width, height) = (garden[0].len(), garden.len());

    for dir in DIRS {
        if !is_out_of_bounds((width, height), (x, y), dir) {
            let (new_x, new_y) = ((x as i32 + dir.0) as usize, (y as i32 + dir.1) as usize);
            if !visited[new_x][new_y] && group_type == garden[new_x][new_y] {
                garden_group.push((new_x, new_y));
                find_group(garden, visited, garden_group, (new_x, new_y), group_type);
            }
        }
    }
}

fn parse_garden(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .split("\n")
        .map(|str| str.chars().collect::<Vec<_>>())
        .collect()
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
