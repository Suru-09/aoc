use std::collections::{HashMap, HashSet};

const INPUT: &str = "input.txt";
const _EXAMPLE: &str = "example.txt";

pub fn solve_part_1() {
    let map_str = utils::read_file_lines(INPUT);
    let (width, height, map) = parse_map(&map_str);
    let points = collect_points(&map, width, height);

    let mut unique_antinodes = HashSet::new();
    points.iter().for_each(|(_, value)| {
        for i in 0..value.len() - 1 {
            for j in i + 1..value.len() {
                let (x1, y1) = (value[i].x as isize, value[i].y as isize);
                let (x2, y2) = (value[j].x as isize, value[j].y as isize);

                let (upper_x, upper_y);
                let (lower_x, lower_y);

                if y2 > y1 {
                    upper_x = x1 - (x2 - x1);
                    upper_y = y1 - (y2 - y1);
                } else {
                    upper_x = x1 - (x2 - x1);
                    upper_y = y1 + (y1 - y2);
                }

                if (upper_x as usize) < height && (upper_y as usize) < width {
                    let upper_point = Point::new(upper_x as usize, upper_y as usize);

                    unique_antinodes.insert(upper_point);
                }

                if y2 > y1 {
                    lower_x = x2 + (x2 - x1);
                    lower_y = y2 + (y2 - y1);
                } else {
                    lower_x = x2 + (x2 - x1);
                    lower_y = y2 - (y1 - y2);
                }

                if (lower_x as usize) < height && (lower_y as usize) < width {
                    let lower_point = Point::new(lower_x as usize, lower_y as usize);
                    unique_antinodes.insert(lower_point);
                }
            }
        }
    });

    let result = unique_antinodes.len();
    println!("Result for part1: {}", result);
}

pub fn solve_part_2() {
    let map_str = utils::read_file_lines(INPUT);
    let (width, height, map) = parse_map(&map_str);
    let points = collect_points(&map, width, height);

    let mut unique_antinodes = HashSet::new();
    points.iter().for_each(|(_, value)| {
        for i in 0..value.len() - 1 {
            for j in i + 1..value.len() {
                let (x1, y1) = (value[i].x as isize, value[i].y as isize);
                let (x2, y2) = (value[j].x as isize, value[j].y as isize);

                unique_antinodes.insert(Point::new(x1 as usize, y1 as usize));
                unique_antinodes.insert(Point::new(x2 as usize, y2 as usize));

                let (mut upper_x, mut upper_y);
                let (mut lower_x, mut lower_y);

                if y2 > y1 {
                    upper_x = x1 - (x2 - x1);
                    upper_y = y1 - (y2 - y1);
                } else {
                    upper_x = x1 - (x2 - x1);
                    upper_y = y1 + (y1 - y2);
                }

                while (upper_x as usize) < height && (upper_y as usize) < width {
                    let upper_point = Point::new(upper_x as usize, upper_y as usize);
                    unique_antinodes.insert(upper_point);

                    if y2 > y1 {
                        upper_x -= x2 - x1;
                        upper_y -= y2 - y1;
                    } else {
                        upper_x -= x2 - x1;
                        upper_y += y1 - y2;
                    }
                }

                if y2 > y1 {
                    lower_x = x2 + (x2 - x1);
                    lower_y = y2 + (y2 - y1);
                } else {
                    lower_x = x2 + (x2 - x1);
                    lower_y = y2 - (y1 - y2);
                }

                while (lower_x as usize) < height && (lower_y as usize) < width {
                    let lower_point = Point::new(lower_x as usize, lower_y as usize);
                    unique_antinodes.insert(lower_point);

                    if y2 > y1 {
                        lower_x += x2 - x1;
                        lower_y += y2 - y1;
                    } else {
                        lower_x += x2 - x1;
                        lower_y -= y1 - y2;
                    }
                }
            }
        }
    });

    let result = unique_antinodes.len();
    println!("Result for part2: {}", result);

    //print_map(&map, &unique_antinodes, width, height);
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

fn collect_points(map: &Vec<u8>, width: usize, height: usize) -> HashMap<char, Vec<Point>> {
    let mut points: HashMap<char, Vec<Point>> = HashMap::new();
    for i in 0..height {
        for j in 0..width {
            let c = map[i * width + j] as char;
            if c.is_digit(10) || c.is_alphabetic() {
                match points.get_mut(&c) {
                    Some(vec) => {
                        vec.push(Point::new(i, j));
                    }
                    None => {
                        points.insert(c, vec![Point::new(i, j)]);
                    }
                }
            }
        }
    }
    points
}

fn parse_map(maze_str: &Vec<String>) -> (usize, usize, Vec<u8>) {
    let mut maze: Vec<String> = maze_str.clone();
    maze.retain(|line| !line.is_empty());
    let height = maze.len();
    let width = maze[0].len();
    let mut map_u8 = Vec::new();
    maze.iter()
        .for_each(|str| map_u8.append(&mut str.as_bytes().to_owned()));

    (width, height, map_u8)
}

fn _print_map(map: &Vec<u8>, antinodes: &HashSet<Point>, width: usize, height: usize) {
    for i in 0..height {
        for j in 0..width {
            let c = map[i * width + j] as char;
            if let Some(_) = antinodes.get(&Point::new(i, j)) {
                if c == '.' {
                    print!("#");
                    continue;
                }
            }
            print!("{}", c);
        }
        println!("");
    }
}

mod utils {
    pub fn _read_file(path: &str) -> String {
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
