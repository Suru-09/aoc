use std::collections::{VecDeque, HashMap};

const INPUT: &str = "input.txt";
const EXAMPLE: &str = "example.txt";

const NUMPAD: [[usize; 3]; 4] = [[7, 8, 9], [4, 5, 6], [1, 2, 3], [usize::MAX, 0, 10]];
const ARROWS: [[char; 3]; 2] = [['E', '^', 'A'], ['<', 'v', '>']];
const DIRS: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

pub fn solve_part_1() {
    let input = utils::read_file_lines(EXAMPLE);
    println!("Input: {:?}", input);
    input.iter().for_each(|code| code.chars().for_each(|ch| {
      let code_part: usize = if ch == 'A' {10} else {
        ch.to_digit(10).unwrap() as usize
      };
      println!("Bfs: {:?}\nCode Part: {}", bfs(code_part), ch);
    }));
}

pub fn solve_part_2() {}

fn is_valid_numpad(curr: (usize, usize), dir: (isize, isize)) -> (bool, (usize, usize)) {
    let (x, y) = (curr.0 as isize, curr.1 as isize);
    let (new_x, new_y) = (x + dir.0, y + dir.1);

    if new_x < 0 || new_x as usize >= NUMPAD.len() || new_y < 0 || new_y as usize >= NUMPAD[0].len()
    {
      return (false, (0, 0));
    }

    if NUMPAD[new_x as usize][new_y as usize] == usize::MAX {
      return (false, (0, 0));
    }

    (true, (new_x as usize, new_y as usize))
}

fn bfs(code_part: usize) -> HashMap<(usize, usize), usize> {
    let mut queue = VecDeque::new();
    let start = (3, 2); // 'A' a.k.a 10 on NUMPAD
    queue.push_back((start, 0));
    let mut visited = HashMap::new();
    visited.insert(start, 0);

    while let Some(((i, j), cost)) = queue.pop_front() {
      if cost > *visited.get(&(i, j)).unwrap() {
        continue;
      }

      if NUMPAD[i][j] == code_part {
        break;
      }

        for d in DIRS {
            let (valid, new_coords) = is_valid_numpad((i, j), d);
            if valid && visited.get(&new_coords).is_none() {
              queue.push_back((new_coords, cost + 1));
              match visited.get_mut(&new_coords) {
                  Some(vis_val) => *vis_val = cost + 1,
                  None => {
                      visited.insert(new_coords, cost + 1);
                  }
              }
            }
        }
    }

    visited
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
    solve_part_1();
    solve_part_2();
}
