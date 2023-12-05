mod part_1 {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    /**
     * @brief Extrtract the first digit and the last digit from a line
     *        concatenates them to form the 'calibration number'.
     * 
     *       Edge cases: if no digits are found, return 0
     *                  if only one digit is found, return that digit 2 times
     */
    fn extract_calibration_number(line: &str) -> i64 {
        let mut calibration_number: i64 = 0;
        for character in line.chars() {
            if character.is_digit(10) {
                calibration_number = character.to_digit(10).unwrap() as i64;
                break;
            }
        }

        for character in line.chars().rev() {
            if character.is_digit(10) {
                calibration_number = calibration_number * 10 + character.to_digit(10).unwrap() as i64;
                break;
            }
        }

        calibration_number
    }

    pub fn solve_part_1() {
        let file = File::open("part1.txt").unwrap();
        let reader = BufReader::new(file);

        let mut total: i64 = 0;
        for line in reader.lines() {
            let line = line.unwrap();
            let calibration_number = extract_calibration_number(&line);
            total += calibration_number;
        }

        println!("Part 1: {}", total);
    }
}

mod part_2 {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    const DIGITS_AS_LETTER_VEC: [&str; 10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    const DIGITS: [i64; 10] = [0, 1, 2, 3, 4, 5, 6, 7, 8 ,9];

    fn extract_calibration_number(line: &str) -> i64 {
        let mut calibration_number: i64 = 0;
        let mut idx: i64 = 0;

        let idx_of_digit_as_string = |digit_as_string: &str| -> i64 {
            for (idx, digit) in DIGITS_AS_LETTER_VEC.iter().enumerate() {
                if digit_as_string.find(digit) != None {
                    return idx as i64;
                }
            }
            -1
        };

        for character in line.chars() {
            idx += 1;
            if character.is_digit(10) {
                break;
            }
        }

        let idx_from_strings: i64 = idx_of_digit_as_string(line);
        
        if idx_from_strings != -1 && idx_from_strings >= idx {
            calibration_number = DIGITS.get(idx_from_strings as usize).unwrap().clone();
        }
        else if idx_from_strings != -1 && idx_from_strings < idx {
            calibration_number = line.chars().nth(idx_from_strings as usize).unwrap().to_digit(10).unwrap() as i64;
        }

        idx = 0;
        for character in line.chars().rev() {
            idx += 1;
            if character.is_digit(10) {
                break;
            }
        }

        let idx_of_digit_as_string_in_reverse = |digit_as_string: &str| -> i64 {
            for (idx, digit) in DIGITS_AS_LETTER_VEC.iter().rev().enumerate() {
                if digit_as_string.find(digit) != None {
                    return idx as i64;
                }
            }
            -1
        };

        let idx_from_strings_in_reverse: i64 = idx_of_digit_as_string_in_reverse(line);

        if idx_from_strings_in_reverse != -1 && idx_from_strings_in_reverse >= idx {
            calibration_number = calibration_number * 10 + DIGITS[idx_from_strings_in_reverse];
        }
        else if idx_from_strings_in_reverse != -1 && idx_from_strings_in_reverse < idx {
            calibration_number = calibration_number * 10 + line.chars().rev().nth(idx_from_strings_in_reverse).unwrap().to_digit(10).unwrap() as i64;
        }

        calibration_number
    }
}


fn main() {
    part_1::solve_part_1();
}
