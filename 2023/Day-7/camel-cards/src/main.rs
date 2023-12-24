
mod part_1 {

use crate::parse_cards_hands;

pub fn solve() {
    parse_cards_hands().iter().for_each(|cards_hand| println!("{:?}", cards_hand));
}

}

mod part_2 {

}

fn read_input() -> Vec<String> {
    std::fs::read_to_string("example.txt")
        .unwrap().lines()
        .map(|line| line.to_string()).collect::<Vec<String>>()
}

fn parse_cards_hands() -> Vec<HandOfCards> {
    read_input().iter().map(|line| {
        let parts = line.split(" ").collect::<Vec<&str>>();
        assert_eq!(parts.len(), 2);

        let cards = parts[0].trim().to_string();
        let cards_bid = parts[1].trim().parse::<u64>().unwrap();
        println!("Cards: {}, cards_bid: {}", cards, cards_bid);
        HandOfCards::new(cards, cards_bid)
    }).collect::<Vec<HandOfCards>>()
}

#[derive(Debug)]
struct HandOfCards {
    cards: String,
    cards_bid: u64
}

impl HandOfCards {
    pub fn new(cards: String, cards_bid: u64) -> Self {
        Self {
            cards,
            cards_bid
        }
    }
}


fn main() {
    part_1::solve();
}
