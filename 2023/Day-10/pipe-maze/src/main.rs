use std::collections::VecDeque;

mod part_1 {
    use crate::find_furthest_pipe;

    pub fn solve() {
        let answer = find_furthest_pipe();
        println!("The answer is: {}", answer);
    }
}

mod part_2 {
    pub fn solve() {

    }
}

#[derive(PartialEq)] #[derive(Debug)]
enum Tile {
    VerticalPipe,
    HorizontalPipe,
    LPipe,
    JPipe,
    SevenPipe,
    FPipe,
    Dot/*(ground) */,
    Start
}

static UP: (i64, i64) = (-1, 0);
static DOWN: (i64, i64) = (1, 0);
static LEFT: (i64, i64) = (0, -1);
static RIGHT: (i64, i64) = (0, 1);

impl Tile {
    /**
     * Return a vector of tuples(first element is
     * the x_offset and the second element is the y offset).
     */
    pub fn get_directions(&self) -> Vec<(i64, i64)> {
        match self {
            Tile::VerticalPipe => vec![UP, DOWN] /* north & south */,
            Tile::HorizontalPipe => vec![LEFT, RIGHT] /* west & east */,
            Tile::LPipe => vec![UP, RIGHT] /* north & east */,
            Tile::JPipe => vec![UP, LEFT]/* north & west */,
            Tile::SevenPipe => vec![DOWN, LEFT]/* south & west */,
            Tile::FPipe => vec![DOWN, RIGHT]/*south & east */,
            Tile::Dot => panic!("Trying to get directions for a DOT????")/* you should never arrive on a dot*/,
            Tile::Start => vec![UP, DOWN, LEFT, RIGHT]/*north, south, west & east*/
        }
    }

    pub fn get_type(tile: &char) -> Tile {
        match tile {
            '|' => Tile::VerticalPipe,
            '-' => Tile::HorizontalPipe,
            'L' => Tile::LPipe,
            'J' => Tile::JPipe,
            '7' => Tile::SevenPipe,
            'F' => Tile::FPipe,
            '.' => Tile::Dot,
            'S' => Tile::Start,
             _ => panic!("Not a tile!!!!!")
        }
    }
}

fn parse_maze() -> Vec<String> {
    std::fs::read_to_string("input.txt")
        .unwrap().lines()
        .map(|line| line.to_string())
        .collect::<Vec<_>>()
}

fn find_start_coordinates(maze: &Vec<String>) -> (usize, usize) {
    for (idx, string) in maze.iter().enumerate() {
        let pos_in_string = string.chars().position(|s| s == 'S');
        if pos_in_string.is_some() {
            return (idx, pos_in_string.unwrap())
        }
    }
    (0, 0)
}

fn find_furthest_pipe() -> usize {
    let maze = parse_maze();
    let mut distances = vec![vec![usize::MAX; maze.first().unwrap().len()]; maze.len()];
    let start_coordinates = find_start_coordinates(&maze);
    println!("Start: {:?}", start_coordinates);
    depth_first_search(&maze, &mut distances, start_coordinates);
    println!("{},{}", maze[0].len(), maze.len());
    //println!("{:?}", distances);
    distances
        .iter()
        .flat_map(|row| row.iter())
        .filter(|&&dist| dist != usize::MAX)
        .cloned()
        .fold(usize::MIN, usize::max)
}

fn is_tile_valid(x: &i64, y: &i64, maze: &Vec<String>, old_coords: (i64, i64)) -> bool {
    if *x < 0 || *y < 0 {
        return false
    }

    let ch = maze[*x as usize].chars().nth(*y as  usize).unwrap();
    if ch == 'S' || ch == '.' {
        return false
    }
        
    if (*x as usize) >= maze.len() ||  (*y as usize) >= maze[0].len() {
        return false
    }

    let tile = Tile::get_type(&ch);
    return can_connect_tiles(&tile, old_coords, (*x, *y))
}

fn can_connect_tiles(new_tile: &Tile, old_coords: (i64, i64), new_coords: (i64, i64)) -> bool {
    if old_coords == (-1, -1) {
        return true
    }
    let (x, y) = new_coords;
    
    println!("{:?}", new_tile);
    for (dx, dy) in new_tile.get_directions().iter() {
        let val_x = x + dx;
        let val_y = y + dy;
        if  val_x == old_coords.0 && val_y == old_coords.1 {
            return true;
        }
    }
    false
}

fn depth_first_search(maze: &Vec<String>, distances: &mut Vec<Vec<usize>>, start_coords: (usize, usize)) {
    let mut stack = VecDeque::new();
    stack.push_back((start_coords, 0));

    while let Some(((x, y), distance)) = stack.pop_back() {
        let ch = maze[x].chars().nth(y).unwrap();
        let tile = Tile::get_type(&ch);

        if distances[x][y] <= distance {
            continue;
        }

        distances[x][y] = distance;
        println!("Distance: {distance}");

        tile.get_directions().iter().for_each(|(dx, dy)| {
            let new_x = x as i64 + dx;
            let new_y = y as i64 + dy;

            let tile_valid = is_tile_valid(&new_x, &new_y, &maze, (x as i64, y as i64));
            println!("Valid Tile? : {tile_valid}");
            if  tile_valid {
                stack.push_back(((new_x as usize, new_y as usize), distance + 1));
            }
        });
    }
}

fn main() {
    part_1::solve();
    part_2::solve();
}
