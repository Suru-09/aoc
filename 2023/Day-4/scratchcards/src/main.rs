#[derive(Debug)]
struct Card {
    number: usize,
    winning_numbers: Vec<usize>,
    scratch_numbers: Vec<usize>,
}

impl Card {
    fn new(number: usize, winning_numbers: Vec<usize>, scratch_numbers: Vec<usize>) -> Self {
        Self { number, winning_numbers, scratch_numbers }
    }

    fn is_winning_number(&self, number: usize) -> bool {
        self.winning_numbers.contains(&number)
    }

    fn is_scratch_number(&self, number: usize) -> bool {
        self.scratch_numbers.contains(&number)
    }

    fn get_cards_points(&self) -> usize {
        let mut points = 0;
        for number in self.scratch_numbers.iter() {
            if self.is_winning_number(*number) {
                if points == 0 {
                    points = 1;
                } else {
                    points *= 2;
                }
            }
        }
        points
    }
}


fn read_input() -> String {
    std::fs::read_to_string("input.txt").unwrap()
}

fn parse_input(input: &str) -> Vec<String> {
    input
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>()
}

fn parse_card(line: &str) -> Card {
    let split = line.split(":").collect::<Vec<&str>>();
    let card_number = split[0].replace("Card ", "").trim().parse::<usize>().unwrap();
    println!("Card number: {}", card_number);

    let winning_numbers_string = split[1].clone().split("|").collect::<Vec<&str>>()[0];
    let winning_numbers = winning_numbers_string.trim().split(" ").collect::<Vec<&str>>()
        .iter()
        .filter(|number| number.len() > 0)
        .map(|number| number.trim().parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    println!("Winning numbers: {:?}", winning_numbers);

    let scratch_numbers_string = split[1].split("|").collect::<Vec<&str>>()[1];
    let scratch_numbers = scratch_numbers_string.trim().split(" ").collect::<Vec<&str>>()
        .iter()
        .filter(|number| number.len() > 0)
        .map(|number| number.trim().parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    println!("Scratch numbers: {:?}", scratch_numbers);

    Card::new(card_number, winning_numbers, scratch_numbers)
}


mod part_1 {
    use crate::{parse_input, read_input, parse_card, Card};

    pub fn solve() {
        let input = read_input();
        let data = parse_input(&input);
        let cards = data.iter().map(|line| parse_card(line)).collect::<Vec<Card>>();
        let mut sum = 0;
        cards.iter().for_each(|card| {
            sum += card.get_cards_points();
        });
        println!("Sum: {}", sum);
    }
}

mod part_2 {

}


fn main() {
    part_1::solve();
}
