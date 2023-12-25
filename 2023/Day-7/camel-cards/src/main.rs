use std::collections::HashMap;

mod part_1 {
use crate::parse_cards_hands;
use std::cmp::Ordering;

pub fn solve() {
    let mut cards_vec = parse_cards_hands();
    cards_vec.sort_by(|hand_1, hand_2| {
        if hand_1.hand_type == hand_2.hand_type {
            return match hand_1.is_stronger_than(hand_2) {
                true  => Ordering::Greater,
                false => Ordering::Less,
            }
        }
        else {
            if hand_1.hand_type.value() > hand_2.hand_type.value() {
                return Ordering::Greater;
            }
            else {
                return Ordering::Less;
            }
        }
    });
    let mut sum: u64 = 0;
    for (rank, cards_hand) in cards_vec.iter().enumerate() {
        sum += (rank as u64 + 1) * cards_hand.cards_bid;
    }
    println!("Sum for part1 is: {}", sum);
}
}

mod part_2 {
    use crate::parse_cards_hands;
    use std::cmp::Ordering;
    
    pub fn solve() {
        let mut cards_vec = parse_cards_hands();
        cards_vec.iter_mut().for_each(
            |cards_hand| cards_hand.hand_type = cards_hand.get_hand_type_joker_version(&cards_hand.cards)
        );

        cards_vec.sort_by(|hand_1, hand_2| {
            if hand_1.hand_type == hand_2.hand_type {
                return match hand_1.is_stronger_than_joker_version(hand_2) {
                    true  => Ordering::Greater,
                    false => Ordering::Less,
                }
            }
            else {
                if hand_1.hand_type.value() > hand_2.hand_type.value() {
                    return Ordering::Greater;
                }
                else {
                    return Ordering::Less;
                }
            }
        });
        let mut sum: u64 = 0;
        for (rank, cards_hand) in cards_vec.iter().enumerate() {
            sum += (rank as u64 + 1) * cards_hand.cards_bid;
        }
        println!("Sum for part2 is: {}", sum);
    }
}

fn read_input() -> Vec<String> {
    std::fs::read_to_string("input.txt")
        .unwrap().lines()
        .map(|line| line.to_string()).collect::<Vec<String>>()
}

fn parse_cards_hands() -> Vec<HandOfCards> {
    read_input().iter().map(|line| {
        let parts = line.split(" ").collect::<Vec<&str>>();
        assert_eq!(parts.len(), 2);

        let cards = parts[0].trim().to_string();
        let cards_bid = parts[1].trim().parse::<u64>().unwrap();
        HandOfCards::new(cards, cards_bid)
    }).collect::<Vec<HandOfCards>>()
}

#[derive(Debug)]
struct HandOfCards {
    cards: String,
    cards_bid: u64,
    hand_type: HandType
}

impl HandOfCards {
    pub fn new(cards: String, cards_bid: u64) -> Self {
        let hand_type = Self::get_hand_type(&cards);
        Self {
            cards,
            cards_bid,
            hand_type
        }
    }

    fn get_hand_type(cards: &str) -> HandType {
        let mut map: HashMap<char, u8> = HashMap::new();
        cards.chars().for_each(|ch| {
            *map.entry(ch).or_insert(0) += 1;
        });

        match map.len() {
            5 => HandType::HighCard,
            4 => HandType::OnePair,
            3 => {
                if map.iter().any(|(_, value)| *value == 3) {
                    return HandType::ThreeAKind;
                }
                return HandType::TwoPair;
            },
            2 => {
                if map.iter().any(|(_, value)| *value == 4) {
                    return HandType::FourAKind;
                }
                return HandType::FullHouse;
            },
            1 => HandType::FiveAKind,
            _ => HandType::HighCard,
        }
    }

    fn get_hand_type_joker_version(&self, cards: &str) -> HandType {
        let mut map: HashMap<char, u8> = HashMap::new();
        cards.chars().for_each(|ch| {
            *map.entry(ch).or_insert(0) += 1;
        });
        let joker_count = *map.get(&'J').unwrap_or(&0);
        let substract_jk_count = if joker_count > 0 {1} else {0};

        match map.len() - substract_jk_count {
            5 => HandType::HighCard,
            4 => HandType::OnePair,
            3 => {
                if map.iter().any(|(_, value)| (*value + joker_count) == 3) {
                    return HandType::ThreeAKind;
                }
                return HandType::TwoPair;
            },
            2 => {
                if map.iter().any(|(_, value)| (*value + joker_count) == 4) {
                    return HandType::FourAKind;
                }
                return HandType::FullHouse;
            },
            1 => HandType::FiveAKind,
            0 => HandType::FiveAKind /*It means joker_count == 5(map.len() == 1 -1 -> 0)*/,
            _ => HandType::HighCard,
        }
    }

    fn is_stronger_than(&self, other_hand: &HandOfCards) -> bool {
        for (idx, ch) in self.cards.chars().enumerate() {
           let other_char =  other_hand.cards.chars().nth(idx).unwrap(); 
           if ch !=  other_char {
                if NormalCardType::get_type(ch).value() > NormalCardType::get_type(other_char).value() {
                    return true;
                }
                else {
                    return false;
                }
           }
        }
        false
    }

    fn is_stronger_than_joker_version(&self, other_hand: &HandOfCards) -> bool {
        for (idx, ch) in self.cards.chars().enumerate() {
           let other_char =  other_hand.cards.chars().nth(idx).unwrap(); 
           if ch !=  other_char {
                if JokerCardType::get_type(ch).value() > JokerCardType::get_type(other_char).value() {
                    return true;
                }
                else {
                    return false;
                }
           }
        }
        false
    }
}

#[derive(Debug)]
enum NormalCardType {
    Number(u8),
    T/*???*/,
    Jack,
    Queen,
    King,
    Ace
}

impl NormalCardType {
    fn get_type(card: char) -> NormalCardType {
        match card {
            'T' => NormalCardType::T,
            'J' => NormalCardType::Jack,
            'Q' => NormalCardType::Queen,
            'K' => NormalCardType::King,
            'A' => NormalCardType::Ace,
            digit => NormalCardType::Number(digit.to_digit(10).unwrap() as u8),
        }
    }

    fn value(&self) -> u64 {
        match *self {
            NormalCardType::T => 10,
            NormalCardType::Jack => 11,
            NormalCardType::Queen => 12,
            NormalCardType::King => 13,
            NormalCardType::Ace => 14,
            NormalCardType::Number(digit) => digit as u64
        }
    }
}

#[derive(Debug)]
enum JokerCardType {
    Number(u8),
    T/*???*/,
    Queen,
    King,
    Ace,
    Joker
}

impl JokerCardType {
    fn get_type(card: char) -> JokerCardType {
        match card {
            'T' => JokerCardType::T,
            'Q' => JokerCardType::Queen,
            'K' => JokerCardType::King,
            'A' => JokerCardType::Ace,
            'J' => JokerCardType::Joker,
            digit => JokerCardType::Number(digit.to_digit(10).unwrap() as u8),
        }
    }

    fn value(&self) -> u64 {
        match *self {
            JokerCardType::T => 10,
            JokerCardType::Queen => 12,
            JokerCardType::King => 13,
            JokerCardType::Ace => 14,
            JokerCardType::Joker => 1,
            JokerCardType::Number(digit) => digit as u64,
        }
    }
}

#[derive(Debug)] #[derive(PartialEq)]
enum HandType {
    FiveAKind,
    FourAKind,
    FullHouse,
    ThreeAKind,
    TwoPair,
    OnePair,
    HighCard
}

impl HandType {
    fn value(&self) -> u64 {
        match *self {
            HandType::FiveAKind => 7,
            HandType::FourAKind => 6,
            HandType::FullHouse => 5,
            HandType::ThreeAKind => 4,
            HandType::TwoPair => 3,
            HandType::OnePair => 2,
            HandType::HighCard => 1,
        }
    }
}

fn main() {
    part_1::solve(); // 250347426
    part_2::solve(); // 251224870
}
