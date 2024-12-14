use std::collections::HashMap;

const INPUT: &str = "input.txt";
const EXAMPLE: &str = "example.txt";

pub fn solve_part_1() {
    let robots = parse_robots(EXAMPLE);
    let width = 11;
    let height = 7;
    let positions = robots
        .iter()
        .map(|robot| pos_after_n_seconds(robot, width, height, 100))
        .collect::<Vec<(isize, isize)>>();

    pretty_print(width, height, &positions);
    println!("Positions: {:?}", positions);

    let q_counts = robots_in_quadrants(&positions, width, height);
    println!("Q counts: {:?}", q_counts);
    let result = q_counts[0] * q_counts[1] * q_counts[2] * q_counts[3];
    println!("Result for part1: {}", result);
}

pub fn solve_part_2() {}

fn pretty_print(width: isize, height: isize, positions: &Vec<(isize, isize)>) {
    let mut hash_map = HashMap::new();
    positions
        .iter()
        .for_each(|(x, y)| match hash_map.get_mut(&(*x, *y)) {
            Some(count) => *count += 1,
            None => {
                hash_map.insert((*x, *y), 1);
            }
        });

    let mut matrix = vec![vec!['.'; width as usize]; height as usize];
    for i in 0..height as usize {
        for j in 0..width as usize {
            if i == (height / 2) as usize || j == (width / 2) as usize {
                //matrix[i][j] = ' ';
            }
            if let Some(count) = hash_map.get(&(i as isize, j as isize)) {
                matrix[i][j] = count.to_string().chars().collect::<Vec<char>>()[0];
            }
            print!("{}", matrix[i][j]);
        }
        println!("");
    }
}

fn robots_in_quadrants(positions: &Vec<(isize, isize)>, width: isize, height: isize) -> Vec<usize> {
    let mut q_counts = vec![0; 4];
    positions.iter().for_each(|(x, y)| {
        if *x < width / 2 && *y < height / 2 {
            q_counts[0] += 1;
        }

        if *x < width / 2 && *y > height / 2 {
            q_counts[1] += 1;
        }

        if *x > width / 2 && *y < height / 2 {
            q_counts[2] += 1;
        }

        if *x > width / 2 && *y > height / 2 {
            q_counts[3] += 1;
        }
    });
    q_counts
}

fn pos_after_n_seconds(
    robot: &Robot,
    width: isize,
    height: isize,
    seconds: isize,
) -> (isize, isize) {
    let (px, py) = robot.pos;
    let (vx, vy) = robot.v;

    let (res_x, res_y) = (
        (px + vx * seconds) % width ,
        (py + vy * seconds) % height,
    );

    println!("Seconds: {}, w: {}, h: {}, robot: {:?}", seconds, width, height, robot);
    println!("Res_x: {}, Res_y {}", res_x, res_y);
    println!("Res_x(abs): {}, Res_y(abs) {}", res_x.abs(), res_y.abs());

    (res_x.abs() , res_y.abs())
}

#[derive(Debug, Default)]
struct Robot {
    pub pos: (isize, isize),
    pub v: (isize, isize),
}

impl Robot {
    pub fn new(pos: (isize, isize), v: (isize, isize)) -> Self {
        Self { pos, v }
    }
}

fn parse_robots(path: &str) -> Vec<Robot> {
    utils::read_file_lines(path)
        .into_iter()
        .map(|line| parse_robot(&line))
        .collect()
}

fn collect_coords(input: &str) -> Vec<isize> {
    input
        .split(",")
        .collect::<Vec<&str>>()
        .iter()
        .map(|str| str.trim().parse::<isize>().unwrap())
        .collect::<Vec<isize>>()
}

fn parse_robot(robot_str: &str) -> Robot {
    let pos_and_velo = robot_str.trim().split(" ").collect::<Vec<&str>>();
    assert_eq!(pos_and_velo.len(), 2);
    let pos = collect_coords(&pos_and_velo[0][2..]);
    let velocity = collect_coords(&pos_and_velo[1][2..]);
    Robot::new((pos[0], pos[1]), (velocity[0], velocity[1]))
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
