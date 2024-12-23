use std::collections::{HashMap, HashSet};

const INPUT: &str = "input.txt";
const EXAMPLE: &str = "example.txt";

pub fn solve_part_1() {
    let connections = parse_connections(INPUT);
    let pcs = all_computers(&connections);
    let connections_map = connections_map(&connections);

    let mut result = 0;
    let mut pcs_removed = vec![];
    for pc in &pcs {
        let set1 = connections_map
            .get(&pc)
            .unwrap()
            .iter()
            .filter(|str| !pcs_removed.contains(str))
            .map(|str| str.to_owned())
            .collect::<HashSet<_>>()
            .intersection(&pcs)
            .map(|str| str.to_owned())
            .collect::<HashSet<_>>();
        let mut set1_removed = vec![];

        for el in &set1 {
            let binding = connections_map
                .get(&el)
                .unwrap()
                .iter()
                .filter(|str| !pcs_removed.contains(str))
                .map(|str| str.to_owned())
                .collect::<HashSet<_>>();
            let set1_binding = set1
                .iter()
                .filter(|str| !set1_removed.contains(str))
                .map(|str| str.to_owned())
                .collect::<HashSet<_>>();
            let set2 = binding.intersection(&set1_binding).collect::<HashSet<_>>();

            if pc.starts_with("t") || el.starts_with("t") {
                result += set2.len();
            } else {
                result += set2.iter().fold(
                    0usize,
                    |acc, el| {
                        if el.starts_with("t") {
                            acc + 1
                        } else {
                            acc
                        }
                    },
                );
            }

            set1_removed.push(el);
        }

        pcs_removed.push(pc);
    }

    println!("Result for part1: {}", result);
}

pub fn solve_part_2() {
  let connections = parse_connections(INPUT);
  let pcs = all_computers(&connections);
  let connections_map = connections_map(&connections);

  let mut connected_sets = HashSet::new();
  let largest_network = lan_party(&pcs, &connections_map, &mut connected_sets);

  let mut network_v = largest_network.iter().collect::<Vec<_>>();
  network_v.sort();
  print!("Result for part2: ");
  for el in network_v {
    print!("{},", el);
  }
  println!("");
}

fn lan_party(pcs: &HashSet<String>, conns: &HashMap<&String, HashSet<String>>, connected_sets: &mut HashSet<String>) -> HashSet<String> {
  if pcs.is_empty() {
    return connected_sets.clone();
  }

  let mut largest = connected_sets.clone();

  let mut pcs_removed = vec![];
  for pc in pcs {
    connected_sets.insert(pc.clone());
    let set = conns
        .get(&pc)
        .unwrap()
        .iter()
        .filter(|str| !pcs_removed.contains(*str))
        .map(|str| str.to_owned())
        .collect::<HashSet<_>>()
        .intersection(&pcs)
        .map(|str| str.to_owned())
        .collect::<HashSet<_>>();

    let resulting_set = lan_party(&set, conns, connected_sets);

    if resulting_set.len() > largest.len() {
      largest = resulting_set.clone();
    }

    connected_sets.remove(pc);
    pcs_removed.push(pc.clone());
  }

  largest
}

fn parse_connections(input: &str) -> HashSet<(String, String)> {
    utils::read_file_lines(input)
        .iter()
        .map(|line| {
            let pcs = line.trim().split("-").collect::<Vec<&str>>();
            assert_eq!(pcs.len(), 2);
            (pcs[0].to_string(), pcs[1].to_string())
        })
        .collect::<HashSet<(String, String)>>()
}

fn connections_map(connections: &HashSet<(String, String)>) -> HashMap<&String, HashSet<String>> {
    let mut all_threes: HashMap<&String, HashSet<String>> = HashMap::new();
    for conn in connections {
        match all_threes.get_mut(&conn.0) {
            Some(hash_set) => {
                hash_set.insert(conn.1.clone());
            }
            None => {
                let mut new_hash = HashSet::new();
                new_hash.insert(conn.1.clone());
                all_threes.insert(&conn.0, new_hash);
            }
        }

        match all_threes.get_mut(&conn.1) {
            Some(hash_set) => {
                hash_set.insert(conn.0.clone());
            }
            None => {
                let mut new_hash = HashSet::new();
                new_hash.insert(conn.0.clone());
                all_threes.insert(&conn.1, new_hash);
            }
        }
    }
    all_threes
}

fn all_computers(connections: &HashSet<(String, String)>) -> HashSet<String> {
    connections
        .iter()
        .map(|(c1, c2)| {
            let mut pcs = HashSet::new();
            pcs.insert(c1.clone());
            pcs.insert(c2.clone());
            pcs
        })
        .flatten()
        .collect::<HashSet<String>>()
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
