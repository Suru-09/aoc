mod part_1 {
    use crate::{read_input, parse_line};

    pub fn solve() {
        read_input().iter().for_each(|x| {
            println!("{}", x);
            parse_line(x);
        });
    }
}

mod part_2 {
    pub fn solve() {

    }
}

fn read_input() -> Vec<String> {
    std::fs::read_to_string("input.txt")
        .unwrap().lines()
        .map(|line|line.to_string())
        .collect::<Vec<String>>()
}

fn parse_line(line: &str) -> (String, Vec<usize>) {
    let split_arr = line.split(" ").collect::<Vec<&str>>();    // split after space
    assert!(split_arr.len() == 1 || split_arr.len() == 2);

    let spring_conditions: String = s
    ("".to_string(), Vec::new())
}



fn main() {
    part_1::solve();
    part_2::solve();
}
