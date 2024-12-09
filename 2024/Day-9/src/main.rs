const INPUT: &str = "input.txt";
const _EXAMPLE: &str = "example.txt";

pub fn solve_part_1() {
    let disk_str = utils::read_file(INPUT);
    let mut disk = parse_disk(disk_str.trim());
    compact_disk(&mut disk);
    let result = calc_checksum(&disk);
    println!("Result for part1: {}", result);
}

pub fn solve_part_2() {
    let disk_str = utils::read_file(INPUT);
    let mut disk = parse_disk_as_tuples(disk_str.trim());
    compact_disk_by_block(&mut disk);
    let result = calc_checksum_block_disk(&disk);
    println!("Result for part2: {}", result);
}

fn calc_checksum(compacted_disk: &Vec<isize>) -> usize {
    let mut idx = 0;
    compacted_disk
        .iter()
        .fold(0usize, |sum, val| {
            let multipl = val * idx;
            idx += 1;
            if *val == - 1 {
                sum
            } else {
                sum + multipl as usize
            }
        })
}

fn calc_checksum_block_disk(compacted_disk: &Vec<(usize, usize)>) -> usize {
    let mut idx = 0;
    compacted_disk
        .iter()
        .fold(0usize, |sum, (value, count)| {
            sum + (0..*count).fold(0usize, |inner_sum, _: usize| {
                idx += 1;
                if *value > 0 {
                    inner_sum + ((value - 1) * (idx - 1))
                } else {
                    inner_sum
                }
            })
        })
}

fn compact_disk(disk: &mut Vec<isize>) {
    loop {
        let empty_pos: Option<usize> = disk.iter().position(|x| *x == -1);
        match empty_pos {
            Some(pos) => {
                let last_element = disk.pop().expect("Should be");
                if last_element != -1 {
                    disk[pos] = last_element;
                }
            }
            None => break,
        }
    }
}

fn compact_disk_by_block(disk: &mut Vec<(usize, usize)>) {
    for i in (0..disk.len()).rev() {
        for j in 0..i {
            let (i_data, i_count) = disk[i];    // i should contain data.
            let (j_data, j_count) = disk[j];    // j should contain free blocks.

            if i_data > 0 && j_data == 0 && i_count <= j_count {
                disk[i] = (0, i_count);
                disk[j] = (0, j_count - i_count);
                disk.insert(j, (i_data, i_count));
            }
        }
    }
}

fn parse_disk_as_tuples(input: &str) -> Vec<(usize, usize)> {
    let mut vec = vec![];
    for (idx, c) in input.chars().enumerate() {
        let num: usize = c.to_digit(10).unwrap() as usize;
        if idx % 2 == 0 {
            vec.push((idx / 2 + 1, num));
        } else {
            vec.push((0, num));
        }

    }
    vec
}

fn parse_disk(input: &str) -> Vec<isize> {
    let mut vec = vec![];
    for (idx, c) in input.chars().enumerate() {
        let num: isize = c.to_digit(10).unwrap() as isize;
        for _ in 0..num {
            if idx % 2 == 0 {
                vec.push((idx / 2) as isize);
            } else {
                vec.push(-1);
            }
        }
    }
    vec
}

mod utils {
    pub fn read_file(path: &str) -> String {
        std::fs::read_to_string(path).unwrap()
    }

    pub fn _read_file_lines(path: &str) -> Vec<String> {
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
