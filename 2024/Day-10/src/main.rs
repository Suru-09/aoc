const INPUT: &str = "input.txt";
const EXAMPLE: &str = "example.txt";

pub fn solve_part_1() {
    let trail_str = utils::read_file(INPUT);
    let trail = parse_trail(trail_str.trim());
    let trailheads = find_trailheads(&trail);
    let trailheads_score = trailheads_score(&trail, &trailheads, false);
    println!("Result for part1: {}", trailheads_score);
}

pub fn solve_part_2() {
    let trail_str = utils::read_file(INPUT);
    let trail = parse_trail(trail_str.trim());
    let trailheads = find_trailheads(&trail);
    let trailheads_score = trailheads_score(&trail, &trailheads, true);
    println!("Result for part2: {}", trailheads_score);
}

fn out_of_bounds(trail: &Vec<Vec<usize>>, old: (usize, usize), offset: (isize, isize)) -> bool {
    let (width, height) = (trail[0].len(), trail.len());
    let (new_x, new_y) = (old.0 as isize + offset.0, old.1 as isize + offset.1);
    if new_x >= 0 && new_x < height as isize && new_y >= 0 && new_y < width as isize {
        false
    } else {
        true
    }
}

fn trailheads_score(
    trail: &Vec<Vec<usize>>,
    trailheads: &Vec<(usize, usize)>,
    ignore_unique: bool,
) -> usize {
    const DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    trailheads.iter().fold(0usize, |trailheads_score, (i, j)| {
        let mut stack = vec![];
        let mut score = 0;
        stack.push((*i, *j));
        let mut visited = vec![vec![false; trail[0].len()]; trail.len()];

        while !stack.is_empty() {
            let (x, y) = stack.pop().unwrap();

            if trail[x][y] == 9 && (!visited[x][y] || ignore_unique) {
                visited[x][y] = true;
                score += 1;
            }

            for dir in DIRS.iter() {
                if !out_of_bounds(trail, (x, y), *dir) {
                    let (new_x, new_y) =
                        ((x as isize + dir.0) as usize, (y as isize + dir.1) as usize);
                    if trail[new_x][new_y] == trail[x][y] + 1 {
                        stack.push((new_x, new_y));
                    }
                }
            }
        }
        trailheads_score + score
    })
}

fn parse_trail(input: &str) -> Vec<Vec<usize>> {
    input
        .trim()
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|str| {
            str.chars()
                .map(|ch| ch.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>()
}

fn find_trailheads(trail: &Vec<Vec<usize>>) -> Vec<(usize, usize)> {
    trail
        .iter()
        .enumerate()
        .map(|(i, vec)| {
            vec.iter()
                .enumerate()
                .map(|(j, val)| if *val == 0 { Some((i, j)) } else { None })
                .collect::<Vec<Option<(usize, usize)>>>()
                .iter()
                .filter(|x| x.is_some())
                .map(|x| x.unwrap())
                .collect::<Vec<(usize, usize)>>()
        })
        .collect::<Vec<Vec<(usize, usize)>>>()
        .into_iter()
        .flatten()
        .collect::<Vec<(usize, usize)>>()
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
    solve_part_1(); // 746
    solve_part_2(); // 1541
}
