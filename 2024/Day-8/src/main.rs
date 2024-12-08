const INPUT: &str = "input.txt";
const EXAMPLE: &str = "example.txt";

pub fn solve_part_1() {
    let map_str = utils::read_file(EXAMPLE);
    let map = parse_map(&map_str);
    map.iter().for_each(|byte| print!(" {}", *byte as char));
}

pub fn solve_part_2() {}

fn parse_map(maze_str: &str) -> Vec<u8> {
    let mut chars: Vec<char> = maze_str.trim().chars().collect();
    chars.retain(|ch| *ch != '\n');
    let s: String = chars.into_iter().collect();
    (&s).as_bytes()
        .iter()
        .map(|byte| byte.clone())
        .collect::<Vec<u8>>()
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
