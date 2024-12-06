use std::collections::HashSet;

const INPUT: &str = "input.txt";
const EXAMPLE: &str = "example.txt";

const START: char = '^';
const OBSTACLE: char = '#';
const EMPTY: char = '.';

pub fn solve_part_1() {
    let maze_lines = utils::read_file_lines(INPUT);
    let maze = parse_maze(&maze_lines);
    let start = find_start(&maze);
    let mut guard_route_state = HashSet::new();

    match start {
        Some(start) => predict_guard_path(&maze, start, &mut guard_route_state),
        None => unreachable!(),
    };

    let result = guard_route_state
        .iter()
        .map(|(x, y, _)| (x, y))
        .collect::<HashSet<_>>()
        .len();

    println!("Result for part1: {}", result);
}

pub fn solve_part_2() {
    let maze_lines = utils::read_file_lines(INPUT);
    let mut maze = parse_maze(&maze_lines);
    let start = find_start(&maze).unwrap();
    let mut guard_route_state = HashSet::new();

    predict_guard_path(&maze, start, &mut guard_route_state);
    let binding = guard_route_state.clone();
    let unique_route = binding
        .iter()
        .map(|(x, y, _)| (x, y))
        .collect::<HashSet<_>>();
    guard_route_state.clear();

    let mut loops = 0;
    for (x, y) in unique_route.iter() {
        let (i, j) = (**x as usize, **y as usize);
        if (**x, **y) == start || maze[i][j] == OBSTACLE {
            continue;
        }
        maze[i][j] = OBSTACLE;
        if predict_guard_path(&maze, start, &mut guard_route_state) {
            loops += 1;
        }
        guard_route_state.clear();
        maze[i][j] = EMPTY;
    }

    println!("Result for part2: {}", loops);
}

fn print_maze(maze: &Vec<Vec<char>>) {
    maze.iter().for_each(|row| {
        row.iter().for_each(|ch| print!("{}", ch));
        println!("\n");
    });
}

fn out_of_bound(maze: &Vec<Vec<char>>, pos: (isize, isize)) -> bool {
    let (x, y) = pos;
    let row_len = maze.len();
    let col_len = maze[0].len();

    x < 0 || x >= row_len as isize || y < 0 || y >= col_len as isize
}

fn find_start(maze: &Vec<Vec<char>>) -> Option<(isize, isize)> {
    for (row_idx, row) in maze.iter().enumerate() {
        match row.iter().position(|ch| *ch == START) {
            Some(pos) => return Some((row_idx as isize, pos as isize)),
            None => (),
        }
    }
    None
}

fn predict_guard_path(
    maze: &Vec<Vec<char>>,
    starting_pos: (isize, isize),
    guard_route_state: &mut HashSet<(isize, isize, char)>,
) -> bool {
    let (mut new_x, mut new_y): (isize, isize) = (starting_pos.0, starting_pos.1);
    let mut direction = maze[new_x as usize][new_y as usize];

    loop {
        let started_looping = guard_route_state.get(&(new_x, new_y, direction));
        if let Some(_) = started_looping {
            return true;
        }

        // register the route of the guard and the state
        // (direction in which the guard goes next).
        guard_route_state.insert((new_x, new_y, direction));

        let (x, y) = (new_x, new_y);
        match direction {
            '>' => new_y = new_y + 1,
            '<' => new_y = new_y - 1,
            '^' => new_x = new_x - 1,
            'v' => new_x = new_x + 1,
            _ => unreachable!("Guard character('^><v') was not set on current position"),
        };

        if out_of_bound(maze, (new_x, new_y)) {
            return false;
        }

        while maze[new_x as usize][new_y as usize] == OBSTACLE {
            match direction {
                '>' => {
                    new_x = x + 1;
                    new_y = y;
                    direction = 'v';
                }
                '<' => {
                    new_x = x - 1;
                    new_y = y;
                    direction = '^';
                }
                '^' => {
                    new_y = y + 1;
                    new_x = x;
                    direction = '>';
                }
                'v' => {
                    new_y = y - 1;
                    new_x = x;
                    direction = '<';
                },
                _ => unreachable!("Guard character('^><v') was not set on current position"),
            }
        }
    }
}

fn parse_maze(maze: &Vec<String>) -> Vec<Vec<char>> {
    maze.iter()
        .filter(|str| !str.is_empty())
        .map(|row| row.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
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
    solve_part_1(); // 4711
    solve_part_2(); //
}
