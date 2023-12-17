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

   const DIGIT_TO_STR_MAP: [(usize, &str); 9] = [
        (1, "one"),
        (2, "two"),
        (3, "three"),
        (4, "four"),
        (5, "five"),
        (6, "six"),
        (7, "seven"),
        (8, "eight"),
        (9, "nine")
   ];

   fn get_digit_after_digit_as_string(str: String) -> usize {
        DIGIT_TO_STR_MAP.iter().find(|digit_tuple| {
            str.contains(digit_tuple.1)
        }).unwrap().0
   }

   fn extract_calibration_number(line: &str) -> i64 {
        let mut calibration_number: i64 = i64::MIN;
        let mut line_as_string = line.to_string();

        let mut digits_string: Vec<(usize, String)> = Vec::new();
        for (digit, digit_as_string) in DIGIT_TO_STR_MAP.iter() {
            if line_as_string.contains(digit_as_string) {
                let digit_index = line_as_string.find(digit_as_string).unwrap();
                digits_string.push((digit_index, digit_as_string.to_string()));
            }
        }

        digits_string.sort_by(|a, b| a.0.cmp(&b.0));
        
        // replace only first and last occurence of digit string
        if  digits_string.len() > 0 {
            let first_digit_as_string = digits_string.first().unwrap().1.to_string();
            let digit_after_digit_as_string = get_digit_after_digit_as_string(first_digit_as_string.clone());

            line_as_string = line_as_string.replacen(&first_digit_as_string, &digit_after_digit_as_string.to_string(), 1);
            if digits_string.len() > 1 {
                let last_digit_as_string = digits_string.last().unwrap().1.to_string();
                let digit_after_digit_as_string = get_digit_after_digit_as_string(last_digit_as_string.clone());

                line_as_string = line_as_string.replacen(&last_digit_as_string, &digit_after_digit_as_string.to_string(), 1);
            }
        }

        println!("{} -> {}", line, line_as_string);

        for character in line_as_string.chars() {
            if character.is_digit(10) && calibration_number==i64::MIN {
                calibration_number = character.to_digit(10).unwrap() as i64;
                for character in line_as_string.chars().rev() {
                    if character.is_digit(10) {
                        calibration_number = calibration_number * 10 + character.to_digit(10).unwrap() as i64;
                        break;
                    }
                }
            }
        }

        if calibration_number == i64::MIN {
            calibration_number = 0;
        }
        

        calibration_number
   }    

   pub fn solve_part_2() {
        let file = File::open("part2.txt").unwrap();
        let reader = BufReader::new(file);

        let mut total: i64 = 0;
        for line in reader.lines() {
            let line = line.unwrap();
            let calibration_number = extract_calibration_number(&line);
            total += calibration_number;
        }

        println!("Part 2: {}", total);
   }
}


fn main() {
    part_1::solve_part_1();
    part_2::solve_part_2();
}