const INPUT: &str = "input.txt";
const EXAMPLE: &str = "example.txt";
const XMAS: &str = "XMAS";
const XMAS_REVERSED: &str = "SAMX";

fn get_diag(start: (usize, usize), matrix: &Vec<Vec<char>>) -> String {
    let c_size = matrix[0].len();
    let l_size = matrix.len();
    let mut diag = vec![];

    for offset in 0..std::cmp::max(l_size, c_size) {
        let (new_x, new_y) = (start.0 + offset, start.1 + offset);
        if new_x < l_size && new_y < c_size {
            diag.push(matrix[new_x][new_y]);
        } else {
            break;
        }
    }
    diag.into_iter().collect()
}

fn get_anti_diag(start: (usize, usize), matrix: &Vec<Vec<char>>) -> String {
    let c_size = matrix[0].len();
    let l_size = matrix.len();
    let mut anti_diag = vec![];

    for offset in 0..std::cmp::max(l_size, c_size) {
        if offset + start.0 >= l_size || offset > start.1 {
            break;
        }

        let (new_x, new_y) = (start.0 + offset, start.1 - offset);
        anti_diag.push(matrix[new_x][new_y]);
    }
    anti_diag.into_iter().collect()
}

pub fn solve_part_1() {
    let input = utils::read_file(INPUT);
    let matrix = parse_matrix(&input);

    let mut lines_to_search: Vec<String> = matrix
        .iter()
        .map(|line| String::from_iter(line.iter()))
        .collect();

    let mut cols: Vec<String> = vec![];
    for col_idx in 0..matrix[0].len() {
        let mut col = Vec::new();
        for line_idx in 0..matrix.len() {
            col.push(matrix[line_idx][col_idx]);
        }
        cols.push(col.iter().collect());
    }
    lines_to_search.append(&mut cols);

    let c_size = matrix[0].len();
    let l_size = matrix.len();

    let mut diag_start = std::iter::repeat(0_usize)
        .take(c_size)
        .zip(0..c_size)
        .collect::<Vec<(usize, usize)>>();
    diag_start.append(
        &mut (1..l_size)
            .zip(std::iter::repeat(0_usize).take(l_size))
            .collect::<Vec<(usize, usize)>>(),
    );

    let mut diagonals = diag_start
        .iter()
        .map(|start| get_diag(*start, &matrix))
        .filter(|str| str.len() >= XMAS.len())
        .collect::<Vec<_>>();
    lines_to_search.append(&mut diagonals);

    let mut anti_diag_start = std::iter::repeat(0_usize)
        .take(c_size)
        .zip(0..c_size)
        .collect::<Vec<(usize, usize)>>();
    anti_diag_start.append(
        &mut (1..l_size)
            .zip(std::iter::repeat(c_size - 1).take(l_size))
            .collect::<Vec<(usize, usize)>>(),
    );

    let mut anti_diagonals = anti_diag_start
        .iter()
        .map(|start| get_anti_diag(*start, &matrix))
        .filter(|str| str.len() >= XMAS.len())
        .collect::<Vec<_>>();
    lines_to_search.append(&mut anti_diagonals);

    let result = lines_to_search
        .iter()
        .map(|str| {
            str.as_bytes()
                .windows(XMAS.len())
                .filter(|&w| w == XMAS.as_bytes() || w == XMAS_REVERSED.as_bytes())
                .count()
        })
        .collect::<Vec<usize>>()
        .iter()
        .fold(0usize, |sum, counter| sum + counter);

    println!("Result for part1: {}", result);
}

pub fn solve_part_2() {
    let mut result = 0;
    let input = utils::read_file(INPUT);
    let matrix = parse_matrix(&input);

    matrix[..].windows(3).for_each(|window| {
        let it1 = window[0].windows(3);
        let it2 = window[1].windows(3);
        let it3 = window[2].windows(3);

        let zipped = it1
            .into_iter()
            .zip(it2.into_iter())
            .zip(it3.into_iter());

        for ((r1, r2), r3) in zipped {
            let mut is_xmas = r2[1] == 'A';
            let diag = String::from(r1[0].to_string() + &r2[1].to_string() + &r3[2].to_string());
            is_xmas = is_xmas && (diag == "MAS" || diag == "SAM");

            let anti_diag = String::from(r1[2].to_string() + &r2[1].to_string() + &r3[0].to_string());
            is_xmas = is_xmas && (anti_diag == "MAS" || anti_diag == "SAM");

            if is_xmas {
                result += 1;
            }
        }
    });

    println!("Result for part2: {}", result);
}

fn parse_matrix(input: &str) -> Vec<Vec<char>> {
    input
        .split("\n")
        .collect::<Vec<&str>>()
        .into_iter()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>()
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
    solve_part_1(); // 2458
    solve_part_2(); // 1945
}
