use std::fs;

const INPUT: &str = "input.txt";
const EXAMPLE: &str = "example.txt";

mod part_1 {
    pub fn solve_part_1() {}
}

mod part_2 {
    pub fn solve_part_2() {}
}

mod utils {
    fn read_file(path: &str) -> String {
        std::fs::read_to_string(path).unwrap()
    }

    fn read_file_lines(path: &str) -> Vec<string> {
        std::fs::read_to_string(path)
            .unwrap()
            .lines()
            .map(|line| line.to_string())
            .collect::<Vec<_>>()
    }
}

fn main() {
    part_1::solve_part_1();
    part_2::solve_part_2();
}
