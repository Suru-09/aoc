mod part_1 {
    use crate::find_furthest_pipe;

    pub fn solve() {
        find_furthest_pipe();
    }
}

mod part_2 {
    pub fn solve() {

    }
}

fn parse_maze() -> Vec<String> {
    std::fs::read_to_string("example.txt")
        .unwrap().lines()
        .map(|line| line.to_string())
        .collect::<Vec<_>>()
}

fn find_furthest_pipe() -> u64 {
    let mut steps_count = 0;
    let maze = parse_maze();
    let mut distances = vec![vec![0; maze.first().unwrap().len()]; maze.len()];
    println!("{:?}", distances);
    steps_count
}

fn main() {
    part_1::solve();
    part_2::solve();
}
