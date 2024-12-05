const INPUT: &str = "input.txt";
const EXAMPLE: &str = "example.txt";

pub fn solve_part_1() {
    let input_str = utils::read_file(INPUT);
    let printer = parse_printer(&input_str.trim());
    let correct_ordered = printer
        .updates
        .iter()
        .filter(|update| {
            printer.page_order.iter().all(|(p1, p2)| {
                let found_p1 = update.iter().position(|el| el == p1);
                let found_p2 = update.iter().position(|el| el == p2);
                match (found_p1, found_p2) {
                    (Some(idx1), Some(idx2)) => {
                        if idx1 < idx2 {
                            true
                        } else {
                            false
                        }
                    }
                    (_, _) => true,
                }
            })
        })
        .collect::<Vec<&Vec<usize>>>();

    let result = correct_ordered.iter().fold(0usize, |sum, vec| {
        let middle = vec[vec.len() / 2];
        sum + middle
    });

    println!("Result for part1: {}", result);
}

pub fn solve_part_2() {
    let input_str = utils::read_file(INPUT);
    let printer = parse_printer(&input_str.trim());
    let mut incorrect_ordered = printer
        .updates
        .iter()
        .filter(|update| {
            printer.page_order.iter().any(|(p1, p2)| {
                let found_p1 = update.iter().position(|el| el == p1);
                let found_p2 = update.iter().position(|el| el == p2);
                match (found_p1, found_p2) {
                    (Some(idx1), Some(idx2)) => {
                        if idx1 < idx2 {
                            false
                        } else {
                            true
                        }
                    }
                    (_, _) => false,
                }
            })
        })
        .map(|x| x.clone())
        .collect::<Vec<Vec<usize>>>();

    incorrect_ordered.iter_mut().for_each(|update| {
        update.sort_by(|a, b| {
            let is_increasing = printer
                .page_order
                .iter()
                .position(|(x, y)| x == a && y == b);
            let is_decreasing = printer
                .page_order
                .iter()
                .position(|(x, y)| x == b && y == a);
            if let Some(_) = is_increasing {
                return std::cmp::Ordering::Less;
            }
            if let Some(_) = is_decreasing {
                return std::cmp::Ordering::Greater;
            }
            std::cmp::Ordering::Less
        });
    });

    let result = incorrect_ordered.iter().fold(0usize, |sum, vec| {
        let middle = vec[vec.len() / 2];
        sum + middle
    });

    println!("Result for part2: {}", result);
}

#[derive(Debug)]
struct Printer {
    pub page_order: Vec<(usize, usize)>,
    pub updates: Vec<Vec<usize>>,
}

impl Printer {
    pub fn new(page_order: Vec<(usize, usize)>, updates: Vec<Vec<usize>>) -> Self {
        Self {
            page_order,
            updates,
        }
    }
}

fn parse_printer(input: &str) -> Printer {
    let parts = input.split("\n\n").collect::<Vec<&str>>();
    assert_eq!(parts.len(), 2);

    let page_orders = parts
        .first()
        .unwrap()
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|page_order| {
            let splited_order = page_order.split("|").collect::<Vec<&str>>();
            assert_eq!(splited_order.len(), 2);
            let p1 = splited_order.first().unwrap().parse::<usize>().unwrap();
            let p2 = splited_order.last().unwrap().parse::<usize>().unwrap();
            (p1, p2)
        })
        .collect::<Vec<(usize, usize)>>();

    let updates = parts
        .last()
        .unwrap()
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|update| {
            update
                .split(",")
                .collect::<Vec<&str>>()
                .iter()
                .map(|str| str.trim().parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    Printer::new(page_orders, updates)
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
    solve_part_1(); // 5129
    solve_part_2(); // 4077
}
