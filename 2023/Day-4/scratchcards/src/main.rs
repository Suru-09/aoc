#[derive(Debug)] #[derive(Clone)]
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

    fn count_matching_numbers(&self) -> usize {
        let mut count = 0;
        self.scratch_numbers.iter().for_each(|number| {
            if self.is_winning_number(*number) {
                count += 1;
            }
        });
        count
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
    use std::collections::HashMap;
    use crate::{parse_input, read_input, parse_card};

    pub fn solve() {
        let mut cards: HashMap<usize, usize> = HashMap::new();
        let input = read_input();

        let result = parse_input(&input)
            .iter()
            .map(|line|{
                let card = parse_card(line);
                let count_winning_numbers = card.count_matching_numbers();
                let amount_of_current_card = cards.get(&card.number).unwrap_or(&0) + 1;

                if amount_of_current_card > 0 {
                    let from = card.number + 1;
                    let to = card.number + count_winning_numbers;
                    for winning_id in from..to + 1 {
                        *cards.entry(winning_id).or_insert(0) += amount_of_current_card;
                    }
                }
                
                amount_of_current_card
            })
            .sum::<usize>();

        println!("Result: {}", result);
    }
}


fn main() {
    part_1::solve();    // 25010
    part_2::solve();    // 9924412
}
