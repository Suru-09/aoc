use std::fs::File;
use std::io::{BufRead, BufReader};
use std::fmt;

struct Game {
    game_id: usize,
    max_red: usize,
    max_blue: usize,
    max_green: usize,
}

impl Game {
    pub fn new(game_id: usize) -> Game {
        Game {
            game_id,
            max_red: 0,
            max_blue: 0,
            max_green: 0,
        }
    }

    pub fn is_game_possible(&self, total_red: usize, total_green: usize, total_blue: usize) -> bool {
        let mut result = true;
        if self.max_red > total_red || self.max_blue > total_blue || self.max_green > total_green {
            result = false;
        }
        result
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = format!("Game: {}\n", self.game_id);
        result.push_str(&format!("Max red: {}\n", self.max_red));
        result.push_str(&format!("Max blue: {}\n", self.max_blue));
        result.push_str(&format!("Max green: {}\n", self.max_green));
        write!(f, "{}", result)
    }
}

fn read_file() -> Vec<String> {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut lines = Vec::new();
    for line in reader.lines() {
        lines.push(line.unwrap());
    }
    lines
}

fn parse_line(line: String) -> Game {
    let result = line.trim_start_matches("Game");
    let mut split = result.split(":");

    let game_number = split.next().unwrap().trim();
    let game_number = game_number.parse::<usize>().unwrap();

    let mut game = Game::new(game_number);

    let rounds = split.next().unwrap();
    parse_rounds(rounds.to_string(), &mut game);
    game
}

fn parse_rounds(rounds: String, game: &mut Game) {
    for round in rounds.split(";") {
        parse_round(round.trim(), game);
    }
}

fn parse_round(round: &str, game: &mut Game) {
    for cube in round.split(",") {
        let mut split = cube.trim().split(" ");
        let number = split.next().unwrap().trim();
        let number = number.parse::<usize>().unwrap();
        let color = split.next().unwrap().trim();
        match color {
            "red" => {
                if number > game.max_red {
                    game.max_red = number;
                }
            },
            "blue" => {
                if number > game.max_blue {
                    game.max_blue = number;
                }
            },
            "green" => {
                if number > game.max_green {
                    game.max_green = number;
                }
            },
            _ => println!("Unknown color: {}", color),
        }
        
    }
}

pub fn solve_part_1_and_2() {
    let file_content = read_file();
    let mut sum = 0;
    let mut sum_prod = 0;
    for line in file_content {
        let game = parse_line(line);
        if game.is_game_possible(12, 13, 14) {
            println!("Game {} is possible", game.game_id);
            sum += game.game_id;
        } else {
            println!("Game {} is not possible", game.game_id);
        }
        sum_prod += game.max_red * game.max_blue * game.max_green;
        println!("{}", game);
    }
    println!("Sum of possible games part 1: {}", sum);
    println!("Sum of prod of min values part 2: {}", sum_prod);
}

fn main() {
    solve_part_1_and_2();
}