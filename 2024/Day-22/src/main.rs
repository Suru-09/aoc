const INPUT: &str = "input.txt";
const EXAMPLE: &str = "example.txt";

pub fn solve_part_1() {
  let secret_numbers  = parse_lines(INPUT);
  //println!("Initial secret nums: {:?}", secret_numbers);
  let result = secret_numbers.into_iter().fold(0usize, |acc, mut num|{
    simulate_nth_num(&mut num, 2000);
    acc + num
  });

  println!("Result for part1: {}", result);
}

pub fn solve_part_2() {}

fn simulate_nth_num(secret: &mut usize, n: usize) {
  for _ in 0..n {
    *secret = mix_prune_number(*secret, *secret << 6);
    *secret = mix_prune_number(*secret, *secret >> 5);
    *secret = mix_prune_number(*secret, *secret << 11);
  }
}

fn mix_prune_number(secret_number: usize, number: usize) -> usize {
  (number ^ secret_number) % 16777216
}

fn parse_lines(input: &str) -> Vec<usize> {
    utils::read_file_lines(input)
        .iter()
        .filter(|line| !line.is_empty())
        .map(|line| line.trim().parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
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
