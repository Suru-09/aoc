const INPUT: &str = "input.txt";
const EXAMPLE: &str = "example.txt";

pub fn solve_part_1() {
    let input = utils::read_file(INPUT);
    let (keys, locks) = parse_key_locks(&input);
    let locks_heights = locks
        .iter()
        .map(|lock| locks_to_heights(&lock))
        .collect::<Vec<Vec<usize>>>();
    //println!("Locks heights: {:?}", locks_heights);

    let keys_heights = keys
        .iter()
        .map(|key| keys_to_heights(&key))
        .collect::<Vec<Vec<usize>>>();
    //println!("Keys heights: {:?}", keys_heights);

    let mut result = 0;
    for (l_idx, l) in locks_heights.iter().enumerate() {
        let height = locks[l_idx].len();
        for k in &keys_heights {
          //println!("Lock: {:?}, key: {:?}, height: {}", l, k, height);
            let count = l
                .iter()
                .zip(k.iter())
                .filter(|(&x, &y)| x + y > height - 2)
                .count();
            if count == 0 {
                result += 1;
            }
        }
    }

    println!("Result for part1: {}", result);
}

pub fn solve_part_2() {}

fn locks_to_heights(locks: &Vec<Vec<char>>) -> Vec<usize> {
    let mut heights = vec![];
    for c in 0..locks[0].len() {
        let mut idx = 0;
        while idx < locks.len() && locks[idx][c] == '#' {
            idx += 1;
        }
        if idx > 0 {
          heights.push(idx - 1);
        } else {
          heights.push(0);
        }
    }
    heights
}

fn keys_to_heights(keys: &Vec<Vec<char>>) -> Vec<usize> {
    let mut heights = vec![];
    for c in 0..keys[0].len() {
        let mut idx = keys.len() - 1;
        while idx > 0 && keys[idx][c] == '#' {
            idx -= 1;
        }
        heights.push(keys.len() - idx - 2);
    }
    heights
}

fn parse_key_locks(input: &str) -> (Vec<Vec<Vec<char>>>, Vec<Vec<Vec<char>>>) {
    let key_locks = input
        .trim()
        .split("\n\n")
        .map(|str| {
            str.lines()
                .map(|line| line.chars().collect::<Vec<char>>())
                .collect::<Vec<Vec<char>>>()
        })
        .collect::<Vec<Vec<Vec<char>>>>();
    let keys: Vec<Vec<Vec<char>>> = key_locks
        .iter()
        .filter(|key_lock| {
            !key_lock.first().unwrap().contains(&'#') && !key_lock.last().unwrap().contains(&'.')
        })
        .cloned()
        .collect();
    let locks: Vec<Vec<Vec<char>>> = key_locks
        .clone()
        .iter()
        .filter(|key_lock| {
            !key_lock.first().unwrap().contains(&'.') && !key_lock.last().unwrap().contains(&'#')
        })
        .cloned()
        .collect();
    (keys, locks)
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
