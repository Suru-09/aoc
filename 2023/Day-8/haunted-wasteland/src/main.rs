use std::collections::HashMap;
use std::cmp::max;

mod part_1 {
    use crate::parse_input;
    
    pub fn solve() {
        let wasteland_map = parse_input();
        println!("Steps for first part: {}", wasteland_map.navigate_map_and_return_steps());
    }
}

mod part_2 {
    use crate::parse_input;

    pub fn solve() {
        let wasteland_map = parse_input();
        println!("Steps for second part: {}", wasteland_map.ghost_navigate_and_return_steps());
    }

}

fn read_input() -> Vec<String> {
    std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>()
}

fn parse_input() -> WastelandMap {
    let mut lines_vec = read_input();

    let instructions = lines_vec.first().unwrap().clone().chars().collect::<Vec<char>>();
    lines_vec.remove(0);

    let mut map: HashMap<String, (String, String)> = HashMap::new();
    lines_vec.iter()
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            let node = parse_node(line.as_str());
            map.insert(node.0, node.1);
        });

    WastelandMap::new(instructions, map)
}

fn parse_node(line: &str) -> (String, (String, String)) {
    let first_split = line.split("=").collect::<Vec<&str>>();
    let node_itself = first_split[0].trim();

    let second_split = first_split[1].trim().split(",").collect::<Vec<&str>>();
    let left_node_with_paranthesis = second_split[0].trim();
    let left_node = &left_node_with_paranthesis[1..left_node_with_paranthesis.len()];

    let right_node_with_paranthesis = second_split[1].trim();
    let right_node = &right_node_with_paranthesis[..right_node_with_paranthesis.len() - 1];

    (node_itself.to_string(), (left_node.to_string(), right_node.to_string()))
}

fn gcf(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0{
        return max(a, b)
    }

    if a > b {
        gcf(b, a % b)
    }
    else {
        gcf(a, b % a)
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a * (b / gcf(a, b))
}

struct WastelandMap {
    road_instructions: Vec<char>,
    map: HashMap<String, (String, String)> /*<node_name, (left_node, right_node)>*/
}

impl WastelandMap {
    pub fn new(road_instructions: Vec<char>, map: HashMap<String, (String, String)>) -> Self {
        Self {
            road_instructions,
            map
        }
    }

    pub fn navigate_map_and_return_steps(&self) -> u64 {
        let mut count: u64 = 0;
        let mut current_position = "AAA".to_string();

        loop {
            for instruction in self.road_instructions.iter() {
                count += 1;
                current_position = match instruction {
                    'L' => self.map[&current_position].0.clone(),
                    'R' => self.map[&current_position].1.clone(),
                    _   => panic!("Instruction is neither Left nor Right!")
                };
                if current_position == "ZZZ" {
                    return count;
                }
            }
        }   
    }

    pub fn ghost_navigate_and_return_steps(&self) -> u64 {
        let mut cycle: u64 = 0;
        let mut cycle_vec = vec![]; // gets a new value each time we arrive to an end
                                    // the value is represente by how many times we iterated
                                    // through road_instructions and lcm.
        let mut current_positions = self.map.iter()
            .filter(|(node, _)| node.ends_with('A'))
            .map(|(node, _)| node.clone())
            .collect::<Vec<String>>();
        println!("Starting nodes: {:?}", current_positions);

        loop {
            for instr in self.road_instructions.iter() {
                match instr {
                    'L' => {
                        current_positions.iter_mut()
                            .for_each(|node| *node = self.map[node].0.clone())
                    },
                    'R' => {
                        current_positions.iter_mut()
                            .for_each(|node| *node = self.map[node].1.clone())
                    }
                    _ => panic!("Instruction is neither Left nor Right!")
                }
            }

            cycle += 1;
            let positions_old_len = current_positions.len();
            current_positions = current_positions.iter()
                .filter_map(|node| {
                    if node.ends_with('Z') {
                        None
                    }
                    else {
                        Some(node.clone())
                    }
                }).collect();

            if positions_old_len != current_positions.len() {
                cycle_vec.push(cycle);
            }

            if current_positions.len() == 0 {
                break
            }
        }

        cycle_vec.iter()
            .fold(1_u64, |_lcm, &cycle| {
                lcm(_lcm, cycle)
            }) * self.road_instructions.len() as u64
    }
}


fn main() {
    part_1::solve(); // 11911
    part_2::solve(); // 10151663816849
}
