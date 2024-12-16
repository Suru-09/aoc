use std::collections::HashMap;
use std::collections::VecDeque;

const INPUT: &str = "input.txt";
const EXAMPLE: &str = "example.txt";

const DIRS: [(isize, isize); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

pub fn solve_part_1() {
    let maze_str = utils::read_file(EXAMPLE);
    let maze = parse_maze(&maze_str);
    let (start, end) = (find_symbol(&maze, 'S'), find_symbol(&maze, 'E'));
    let visited = lowest_score(&maze, start);
    let mut result = usize::MAX;
    for (key, val) in visited.iter() {
        if key.0 == end.0 && key.1 == end.1 {
            result = std::cmp::min(result, *val);
        }
    }

    println!("Result for part1: {}", result);
}

pub fn solve_part_2() {
    let maze_str = utils::read_file(EXAMPLE);
    let maze = parse_maze(&maze_str);
    let (start, end) = (find_symbol(&maze, 'S'), find_symbol(&maze, 'E'));
    let visited = lowest_score(&maze, start);
    let mut best_cost = usize::MAX;
    for (key, val) in visited.iter() {
        if key.0 == end.0 && key.1 == end.1 {
            best_cost = std::cmp::min(best_cost, *val);
        }
    }

    let result = recreate_path(&maze, end, &visited, best_cost);
    println!("Result for part2: {}", result);
}

fn find_symbol(maze: &Vec<Vec<char>>, symbol: char) -> (usize, usize) {
    for i in 0..maze.len() {
        for j in 0..maze[i].len() {
            if maze[i][j] == symbol {
                return (i, j);
            }
        }
    }
    (0, 0)
}

fn is_out_of_bounds(
    sizes: (usize, usize),
    old: (usize, usize),
    dir: usize,
    backwards: bool,
) -> bool {
    let (width, height) = (sizes.0 as isize, sizes.1 as isize);
    let (old_x, old_y) = (old.0 as isize, old.1 as isize);
    let (off_x, off_y) = DIRS[dir];

    if backwards {
        if old_x - off_x < 0
            || old_x - off_x >= width
            || old_y - off_y < 0
            || old_y - off_y >= height
        {
            return true;
        } else {
            return false;
        }
    }

    if old_x + off_x < 0 || old_x + off_x >= width || old_y + off_y < 0 || old_y + off_y >= height {
        true
    } else {
        false
    }
}

fn lowest_score(
    maze: &Vec<Vec<char>>,
    start: (usize, usize),
) -> HashMap<(usize, usize, usize), usize> {
    let mut queue = VecDeque::new();
    let start_dir = (start.0, start.1, 1);
    queue.push_back((start_dir, 0));
    let (width, height) = (maze[0].len(), maze.len());
    let mut visited = HashMap::new();
    visited.insert(start_dir, 0);

    while let Some(((x, y, dir), cost)) = queue.pop_front() {
        if cost > *visited.get(&(x, y, dir)).unwrap() {
            continue;
        }

        if !is_out_of_bounds((width, height), (x, y), dir, false) {
            let (new_x, new_y) = (
                (x as isize + DIRS[dir].0) as usize,
                (y as isize + DIRS[dir].1) as usize,
            );
            let new_cost = cost + 1;
            let new_pos = ((new_x, new_y, dir), new_cost);

            if maze[new_x][new_y] != '#' {
                match visited.get_mut(&new_pos.0) {
                    Some(visited_val) => {
                        if maze[new_x][new_y] != '#' && *visited_val > new_cost {
                            *visited_val = new_cost;
                            queue.push_back(new_pos);
                        }
                    }
                    None => {
                        visited.insert(new_pos.0, new_pos.1);
                        queue.push_back(new_pos);
                    }
                }
            }
        }

        // turn left/right
        for d in get_left_right(dir).into_iter() {
            if !is_out_of_bounds((width, height), (x, y), d, false) {
                let (new_x, new_y) = (
                    (x as isize + DIRS[d].0) as usize,
                    (y as isize + DIRS[d].1) as usize,
                );
                let new_cost = cost + 1001;
                let new_pos = ((new_x, new_y, d), new_cost);

                if maze[new_x][new_y] != '#' {
                    match visited.get_mut(&new_pos.0) {
                        Some(visited_val) => {
                            if maze[new_x][new_y] != '#' && *visited_val > new_cost {
                                *visited_val = new_cost;
                                queue.push_back(new_pos);
                            }
                        }
                        None => {
                            visited.insert(new_pos.0, new_pos.1);
                            queue.push_back(new_pos);
                        }
                    }
                }
            }
        }
    }

    visited
}

fn recreate_path(
    maze: &Vec<Vec<char>>,
    end: (usize, usize),
    visited: &HashMap<(usize, usize, usize), usize>,
    result_cost: usize,
) -> usize {
    let mut queue = VecDeque::new();
    for ((x, y, dir), cost) in visited {
        if *x == end.0 && *y == end.1 && *cost == result_cost {
            queue.push_back((*x, *y, *dir));
        }
    }

    let (width, height) = (maze[0].len(), maze.len());
    let mut shortest_path = vec![queue[0]];

    while let Some((x, y, dir)) = queue.pop_front() {
        let current_cost = *visited.get(&(x, y, dir)).unwrap();
        println!("Current cost: {}, x: {}, y: {}", current_cost, x, y);

        if !is_out_of_bounds((width, height), (x, y), dir, true) {
            let (new_x, new_y) = (
                (x as isize - DIRS[dir].0) as usize,
                (y as isize - DIRS[dir].1) as usize,
            );
            println!("dir: {} New_x: {}, new_y: {}", dir, new_x, new_y);

            if current_cost > 0 {
                let backwards_cost = current_cost - 1;
                let new_pos = (new_x, new_y, dir);
                println!(
                    "Backwards_cost: {}, visited new: {:?}",
                    backwards_cost,
                    visited.get(&new_pos)
                );
                if visited.get(&new_pos).is_some()
                    && *visited.get(&new_pos).unwrap() == backwards_cost
                {
                    shortest_path.push(new_pos);
                    queue.push_back(new_pos);
                }
            }
        }

        // turn left/right
        for d in get_left_right_backwards(dir).into_iter() {
            if !is_out_of_bounds((width, height), (x, y), d, true) {
                let (new_x, new_y) = (
                    (x as isize - DIRS[d].0) as usize,
                    (y as isize - DIRS[d].1) as usize,
                );

                println!("New_x: {}, new_y: {}", new_x, new_y);

                if current_cost > 0 {
                    let backwards_cost = current_cost - 1001;
                    let new_pos = (new_x, new_y, d);
                    println!(
                        "Backwards_cost: {}, visited new: {:?}",
                        backwards_cost,
                        visited.get(&new_pos)
                    );
                    if visited.get(&new_pos).is_some()
                        && *visited.get(&new_pos).unwrap() == backwards_cost
                    {
                        shortest_path.push(new_pos);
                        queue.push_back(new_pos);
                    }
                }
            }
        }
    }

    print_maze(maze, &shortest_path);
    println!("Shortest path: {:?}", shortest_path);
    println!(
        "Visited: {:?}",
        visited
            .iter()
            .filter_map(|((x, y, dir), _)| if *x == 13 && *y == 13 {
                Some((*x, *y, *dir))
            } else {
                None
            })
            .collect::<Vec<(usize, usize, usize)>>()
    );
    shortest_path.len()
}

fn get_left_right(dir: usize) -> Vec<usize> {
    match dir {
        0 | 2 => vec![1, 3],
        1 | 3 => vec![0, 2],
        _ => panic!("Out of bounds"),
    }
}

fn get_left_right_backwards(dir: usize) -> Vec<usize> {
    match dir {
        0 | 3 => vec![1, 2],
        1 | 2 => vec![0, 3],
        _ => panic!("Out of bounds"),
    }
}

fn parse_maze(input: &str) -> Vec<Vec<char>> {
    input
        .trim()
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|str| str.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>()
}

fn print_maze(maze: &Vec<Vec<char>>, shortest_path: &Vec<(usize, usize, usize)>) {
    let mut maze_local = maze.clone();
    for (x, y, _) in shortest_path {
        maze_local[*x][*y] = 'O';
    }

    for x in 0..maze_local.len() {
        for y in 0..maze_local[x].len() {
            print!("{}", maze_local[x][y]);
        }
        println!("");
    }
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
