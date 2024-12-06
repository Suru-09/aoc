const INPUT: &str = "input.txt";
const EXAMPLE: &str = "example.txt";

const START: char = '^';
const OBSTACLE: char = '#';
const VISITED: char = 'X';
const EMPTY: char = '.';

pub fn solve_part_1() {
    let maze_lines = utils::read_file_lines(INPUT);
    let mut maze = parse_maze(&maze_lines);
    let start = find_start(&maze);

    match start {
        Some(start) => predict_guard_path(&mut maze, start),
        None => unreachable!(),
    }

    //print_maze(&maze);
    let result = maze.iter().fold(0_usize, |sum, row| {
      sum + row.iter().filter(|ch| *ch == &VISITED).count()
    });

    println!("Result for part1: {}", result);
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

fn predict_guard_path(maze: &mut Vec<Vec<char>>, current_pos: (isize, isize)) {
    let (x, y) = current_pos;
    let (mut new_x, mut new_y): (isize, isize) = (x, y);

    let current_ch = maze[x as usize][y as usize];
    maze[x as usize][y as usize] = VISITED;
    let mut next_direction = current_ch;

    match current_ch {
        '>' => new_y = y + 1,
        '<' => new_y = y - 1,
        '^' => new_x = x - 1,
        'v' => new_x = x + 1,
        _ => unreachable!("Guard character('^><v') was not set on current position"),
    };

    if out_of_bound(maze, (new_x, new_y)) {
        return;
    }

    match maze[new_x as usize][new_y as usize] {
        OBSTACLE => match current_ch {
            '>' => {
                new_x = x + 1;
                new_y = y;
                next_direction = 'v';
            }
            '<' => {
                new_x = x - 1;
                new_y = y;
                next_direction = '^';
            }
            '^' => {
                new_y = y + 1;
                new_x = x;
                next_direction = '>';
            }
            'v' => {
                new_y = y - 1;
                new_x = x;
                next_direction = '<';
            }
            _ => panic!("Guard character('^><v') was not set on current position"),
        },
        _ => {}
    }

    maze[new_x as usize][new_y as usize] = next_direction;
    predict_guard_path(maze, (new_x, new_y));
}

pub fn solve_part_2() {}

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
    solve_part_1();
    solve_part_2();
}
