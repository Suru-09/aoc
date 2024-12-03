const INPUT: &str = "input.txt";
const EXAMPLE: &str = "example.txt";

pub fn solve_part_1() {
    let input_str = utils::read_file(INPUT);
    let mul_exprs = parse_mul(&input_str);
    let sum: u64 = mul_exprs.iter().fold(0u64, |sum, (val1, val2)| sum + val1 * val2);
    println!("Sum of the muls is: {}", sum);
}

pub fn solve_part_2() {
    let input_str = utils::read_file(INPUT);
    let mul_do_exprs = parse_mul_and_do(&input_str);
    let sum_mul_do: u64 = mul_do_exprs.iter().fold(0u64, |sum, (val1, val2)| sum + val1 * val2);
     println!("Sum of the mul_dos is: {}", sum_mul_do);
}

fn parse_mul(haystack: &str) -> Vec<(u64, u64)> {
    let mul_re = regex::Regex::new("mul\\(([0-9]{1,3},[0-9]{1,3})\\)").unwrap();
    let mut mul_exprs = vec![];
    for (_, [pair]) in mul_re.captures_iter(haystack).map(|c| c.extract()) {
        let mut parts = pair.split(",");
        let val1 = parts.next().unwrap().parse::<u64>().unwrap();
        let val2 = parts.next().unwrap().parse::<u64>().unwrap();
        mul_exprs.push((val1, val2));
    }
    mul_exprs
}

fn parse_mul_and_do(haystack: &str) -> Vec<(u64, u64)> {
    let mul_re = regex::Regex::new("mul\\(([0-9]{1,3},[0-9]{1,3})\\)|(do\\(\\))|(don't\\(\\))").unwrap();
    let mut mul_exprs: Vec<(u64, u64)> = vec![];
    let mut is_mul_enabled = true;
    for (_, [pair]) in mul_re.captures_iter(haystack).map(|c| c.extract()) {
        if pair.contains("do()") {
            is_mul_enabled = true;
        } else if pair.contains("don't()") {
            is_mul_enabled = false;
        } else {
            if is_mul_enabled {
                let mut parts = pair.split(",");
                let val1 = parts.next().unwrap().parse::<u64>().unwrap();
                let val2 = parts.next().unwrap().parse::<u64>().unwrap();
                mul_exprs.push((val1, val2));
            }
        }
    }
    mul_exprs
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
    solve_part_1(); // 183788984
    solve_part_2(); // 62098619
}
