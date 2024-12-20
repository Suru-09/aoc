use std::collections::{HashMap, HashSet, VecDeque};

const INPUT: &str = "input.txt";
const EXAMPLE: &str = "example.txt";

const DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];

pub fn solve_part_1() {
    println!("Result for part1: {}", solve(100, 2));
}

pub fn solve_part_2() {
    println!("Result for part2: {}", solve(100, 20));
}

fn solve(picos_saved: usize, cheat: usize) -> usize {
    let map_str = utils::read_file(INPUT);
    let map = parse_map(&map_str);
    let (_, path) = bfs(&map).expect("You promised there would be a path..");

    let sizes = (map[0].len(), map.len());
    path.iter()
        .map(|(i, j, cost1)| {
            let adjs = adjacencies((*i, *j), sizes, cheat);
            adjs.into_iter()
                .filter(
                    |adj| match path.iter().find(|&x| x.0 == adj.0 && x.1 == adj.1) {
                        Some((_, _, cost2)) => {
                            let manhattan = i.abs_diff(adj.0) + j.abs_diff(adj.1);
                            if *cost2 > cost1 + 2 && *cost2 - (cost1 + manhattan) >= picos_saved {
                                true
                            } else {
                                false
                            }
                        }
                        None => false,
                    },
                )
                .count()
        })
        .sum()
}

fn is_valid(
    map: &Vec<Vec<char>>,
    blocks: &HashSet<(usize, usize)>,
    old: (usize, usize),
    dir: (isize, isize),
) -> (bool, (usize, usize)) {
    let (width, height) = (map[0].len() as isize, map.len() as isize);
    let (new_x, new_y) = (old.0 as isize + dir.0, old.1 as isize + dir.1);
    if new_x < 0 || new_y < 0 || new_x >= height || new_y >= width {
        return (false, (0, 0));
    }

    if blocks.contains(&(new_x as usize, new_y as usize)) {
        return (false, (0, 0));
    }

    (true, (new_x as usize, new_y as usize))
}

fn adjacencies(point: (usize, usize), size: (usize, usize), cheat: usize) -> Vec<(usize, usize)> {
    let mut adjancencies: Vec<(usize, usize)> = vec![];
    let (x, y) = (point.0 as isize, point.1 as isize);
    let (width, height) = (size.0 as isize, size.1 as isize);

    for d in DIRS {
        let (mut new_x, mut new_y) = (x + d.0, y + d.1);
        let mut count = 0;
        while count < cheat && new_x >= 0 && new_x < height && new_y >= 0 && new_y < width {
            adjancencies.push((new_x as usize, new_y as usize));
            new_x += d.0;
            new_y += d.1;
            count += 1;
        }
    }
    adjancencies
}

fn bfs(map: &Vec<Vec<char>>) -> Option<(usize, Vec<(usize, usize, usize)>)> {
    let (start, end) = (
        find_pos(&map, 'S').expect("Should have start!"),
        find_pos(&map, 'E').expect("Should have end!"),
    );
    let blocks = collect_blocks(&map);
    let mut queue = VecDeque::new();
    let mut visited = HashMap::new();
    visited.insert(start, 0);
    queue.push_back((start, 0));

    while let Some(((i, j), cost)) = queue.pop_front() {
        if cost > *visited.get(&(i, j)).unwrap() {
            continue;
        }

        if (i, j) == end {
            break;
        }

        for d in DIRS {
            let (valid, (new_x, new_y)) = is_valid(map, &blocks, (i, j), d);
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

    let mut path = visited
        .iter()
        .map(|((i, j), &cost)| (*i, *j, cost))
        .collect::<Vec<_>>();
    path.sort_by(|a, b| a.2.cmp(&b.2));

    match visited.get(&end) {
        Some(val) => Some((*val, path)),
        None => None,
    }
}

fn collect_blocks(map: &Vec<Vec<char>>) -> HashSet<(usize, usize)> {
    let mut blocks = HashSet::new();
    for (r_idx, row) in map.iter().enumerate() {
        for (c_idx, &val) in row.iter().enumerate() {
            if val == '#' {
                blocks.insert((r_idx, c_idx));
            }
        }
    }
    blocks
}

fn find_pos(map: &Vec<Vec<char>>, symbol: char) -> Option<(usize, usize)> {
    let pos = map.iter().flatten().position(|&s| s == symbol);
    match pos {
        Some(position) => {
            let width = map[0].len();
            Some((position / width, position % width))
        }
        None => None,
    }
}

fn parse_map(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect())
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
