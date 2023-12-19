use std::collections::HashSet;
#[derive(Debug)]
struct Matrix {
    data: Vec<Vec<char>>,
    rows: usize,
    cols: usize,
}

impl Matrix {
    fn new(data: Vec<Vec<char>>) -> Self {
        let rows = data.len() - 1 as usize;
        let cols = data[0].len() - 1 as usize;
        Self { data, rows, cols }
    }

   fn get(&self, row: usize, col: usize) -> Option<char> {
        if row >= self.rows || col >= self.cols {
            return None;
        }
        Some(self.data[row][col])
    }

    fn is_symbol_nearby(&self, row: usize, col: usize) -> bool {
        let directions: Vec<(i32, i32)> = vec![
            (-1, 0), // up
            (1, 0), // down
            (0, -1), // left
            (0, 1), // right
            (-1, -1), // up-left
            (-1, 1), // up-right
            (1, -1), // down-left
            (1, 1), // down-right
        ];

        for (row_offset, col_offset) in directions {
            let new_row = row_offset + row as i32;
            let new_col = col_offset + col as i32;
            if new_row < 0 || new_col < 0 {
                continue;
            }
            if let Some(symbol) = self.get(new_row as usize, new_col as usize) { 
                if self.is_symbol(symbol) {
                    //println!("Value {} near symbol with coords: row:<{}>, col<{}>", symbol, row, col);
                    return true;
                }
            }
        }
        false
    }

    fn does_number_already_exists(&self, number_to_check: &Vec<(usize, usize, usize)>, numbers: &HashSet<Vec<(usize, usize, usize)>>) -> bool {
        numbers.iter().any(|num| {
            num.iter().all(|(row, col, digit)| {
                number_to_check.iter().any(|(row2, col2, digit2)| {
                    if *row == *row2 && *col == *col2 && *digit == *digit2 {
                        return true;
                    }
                    false
                })
            })
        })
    }

    fn how_many_numbers_nearby(&self, row: usize, col: usize, numbers: &Vec<Vec<(usize, usize, usize)>>) -> (usize, HashSet<Vec<(usize, usize, usize)>>) {
        let mut count: usize = 0;
        let mut numbers_near_star = HashSet::new();
        let directions: Vec<(i32, i32)> = vec![
            (-1, 0), // up
            (1, 0), // down
            (0, -1), // left
            (0, 1), // right
            (-1, -1), // up-left
            (-1, 1), // up-right
            (1, -1), // down-left
            (1, 1), // down-right
        ];

        for (row_offset, col_offset) in directions {
            let new_row = row_offset + row as i32;
            let new_col = col_offset + col as i32;
            if new_row < 0 || new_col < 0 {
                continue;
            }

            numbers.iter().for_each(|number| {
                // if any of the numbers coords match the current coords
                if number.iter().any(|(row, col, _)| *row == new_row as usize && *col == new_col as usize) {
                    println!("Number: {:?} near star with coords: row:<{}>, col<{}>", number, row, col);
                    println!("Number already exists: {}", self.does_number_already_exists(number, &numbers_near_star));
                    if !self.does_number_already_exists(number, &numbers_near_star) {
                        count += 1;
                        numbers_near_star.insert(number.clone());
                    }
                }
            });
        }
        (count, numbers_near_star)
    }

    fn is_symbol(&self, value: char) -> bool {
        value != '.' && !value.is_digit(10)
    }

    fn is_star(&self, value: char) -> bool {
        value == '*'
    }

    fn is_symbol_near_number(&self, number: Vec<(usize, usize, usize)>) -> bool {
        for (row, col, _) in number {
            if self.is_symbol_nearby(row, col) {
                return true;
            }
        }
        false
    }

    fn prod_of_numbers(&self, numbers: &HashSet<Vec<(usize, usize, usize)>>) -> usize {
        let mut product: usize = 1;
        numbers.iter().for_each(|number| {
            product *= self.extract_number(number.to_vec());
        });
        product
    }

    fn are_numbers_near_star(&self, stars: &Vec<(usize, usize)>, numbers: &Vec<Vec<(usize, usize, usize)>>) -> usize {
        let mut sum: usize = 0;
        stars.iter()
             .filter(|(row, col)| {
                let (count, _) = self.how_many_numbers_nearby(*row, *col, numbers);
                println!("Count: {} for star with coords: row:<{}>, col<{}>", count, row, col);
                if count == 2 {
                    return true;
                }
                false
             })
             .for_each(|(row, col)| {
                let (_, numbers_near_star) = self.how_many_numbers_nearby(*row, *col, numbers);
                let prod = self.prod_of_numbers(&numbers_near_star);
                println!("Numbers near star: {:?}, prod: {}", numbers_near_star, prod);
                sum += prod;
             });
        sum
    }

    fn extract_number(&self, num: Vec<(usize, usize, usize)>) -> usize {
        let mut result: usize = 0;
        num.iter().for_each(|(_, _, digit)| {
            result = result * 10 + digit;
        });
        result
    }

    fn get_all_numbers(&self) -> Vec<Vec<(usize, usize, usize)>> {
        let mut num_flag = false;
        let mut result = Vec::new();
        let mut current_number = Vec::new();
        for (row, row_data) in self.data.iter().enumerate() {
            for (col, value) in row_data.iter().enumerate() {
                if value.is_digit(10) {
                    if !num_flag {
                        num_flag = true;
                        current_number = Vec::new();
                    }
                    current_number.push((row, col, value.to_digit(10).unwrap() as usize));
                } else {
                    if num_flag {
                        num_flag = false;
                        if current_number.len() > 0 {
                            result.push(current_number.clone());
                        }
                    }
                }
            }
            // do not let numbers overflow to next row !!!!!!
            if num_flag {
                if current_number.len() > 0 {
                    result.push(current_number.clone());
                }
                current_number = Vec::new();
            }
        }
        result
    }

    fn get_all_stars(&self) -> Vec<(usize, usize)> {
        let mut result = Vec::new();
        for (row, row_data) in self.data.iter().enumerate() {
            for (col, value) in row_data.iter().enumerate() {
                if self.is_star(*value) {
                    result.push((row, col));
                }
            }
        }
        result
    }
}
fn parse_input(input: &str) -> Matrix {
    let data = input
        .lines()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();
    Matrix::new(data)
}

fn read_input() -> String {
    std::fs::read_to_string("input.txt").unwrap()
}

mod part_1 {
    use crate::{parse_input, read_input};

    pub fn solve() {
        let input = read_input();
        let matrix = parse_input(&input);
        let numbers = matrix.get_all_numbers();
        let mut sum = 0;
        numbers.iter().filter(|number| {
            matrix.is_symbol_near_number(number.to_vec())
        })
        .for_each(|number| {
            sum += matrix.extract_number(number.to_vec());
        });
        println!("Sum: {}", sum);
    }
}

mod part_2 {
    use crate::{parse_input, read_input};

    pub fn solve() {
        let input = read_input();
        let matrix = parse_input(&input);
        let numbers = matrix.get_all_numbers();
        let stars = matrix.get_all_stars();
        println!("Stars: {:?}", stars);
        println!("Numbers: {:?}", numbers);
        println!("Sum: {}", matrix.are_numbers_near_star(&stars, &numbers));
    }
}

fn main() {
    part_1::solve();    // 527144
    part_2::solve();    // 81463996
}
