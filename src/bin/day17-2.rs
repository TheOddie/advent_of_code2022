use std::collections::{HashSet, HashMap};
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let input = parse_input(include_str!("../input/day17.txt"));

    println!("{}", solve(&input, 1_000_000_000_000));
    println!("Done in {}ms", start.elapsed().as_millis());
}

fn solve(directions: &Vec<Direction>, target: i64) -> i64
{
    let mut rock_count = 0;
    let mut direction_index = 0;
    let mut height = 0;
    
    let mut rock = RockShape::get_nth(rock_count as usize % 5).create_at(height);
    let mut map: HashSet<(i64, i64)> = HashSet::new();
    let mut seen: HashMap<(usize, usize, u128), (i64, i64)> = HashMap::new();
    let mut cycle: Option<Cycle> = None;

    loop {
        match directions[direction_index] {
            Direction::LEFT => rock.move_left(&map),
            Direction::RIGHT => rock.move_right(&map),
        };

        // returns true if it was able to move the rock
        if !rock.move_down(&map) {
            // place rock
            for tile in rock.tiles.iter() {
                height = height.max(tile.1);
                map.insert(*tile);
            }
            rock_count += 1;

            // check if we have seen this rock configuration before
            let key = (direction_index, rock_count as usize % 5, get_key(&map, height));
            match cycle {
                None => {
                    if seen.contains_key(&key) {
                        let (prev_height, prev_rock_count) = *seen.get(&key).unwrap();
                        cycle = Some(Cycle {height: height - prev_height, rock_count: rock_count - prev_rock_count});
                        if (target - rock_count) % prev_rock_count == 0 {
                            break;
                        }
                    } else {
                        seen.insert(key, (height, rock_count));
                    }
                }
                Some(c) => {
                    if (target - rock_count) % c.rock_count == 0 {
                        break;
                    }
                }
            }
            
            rock = RockShape::get_nth(rock_count as usize % 5).create_at(height);
        }

        direction_index = (direction_index + 1) % directions.len();
    }

    let cycle = cycle.unwrap();
    height + cycle.height * ((target - rock_count) / cycle.rock_count)
}

fn get_key(map: &HashSet<(i64, i64)>, height: i64) -> u128 {
    (0..16).map(|y| ((0..7).map(|x| (map.contains(&(x, height - y)) as u8) << x).sum::<u8>() as u128) << y).sum()
}

fn parse_input(s: &str) -> Vec<Direction>
{
    s.chars().map(|ch| {
        match ch {
            '<' => Direction::LEFT,
            '>' => Direction::RIGHT,
            _ => unreachable!()
        }
    }).collect()
}

#[derive(Clone)]
struct RockShape {
    tiles: Vec<(i64, i64)>,
}

#[derive(Copy, Clone, Debug)]
struct Cycle {
    height: i64,
    rock_count: i64
}

impl RockShape {

    fn from(tiles: Vec<(i64, i64)>) -> Self {
        RockShape { tiles }
    }
    
    fn create_at(&mut self, height: i64) -> Self {
        self.tiles.iter_mut().for_each(|(_x, y)| *y += height + 4);
        self.clone()
    }

    fn move_left(&mut self, map: &HashSet<(i64, i64)>) {
        let new_positions: Vec<(i64, i64)> = self.tiles.iter().map(|&(x, y)| (x-1, y)).collect();
        if !new_positions.iter().any(|&(x, y)| x < 0 || map.contains(&(x, y))) {self.tiles = new_positions}
    }

    fn move_right(&mut self, map: &HashSet<(i64, i64)>) {
        let new_positions: Vec<(i64, i64)> = self.tiles.iter().map(|&(x, y)| (x+1, y)).collect();
        if !new_positions.iter().any(|&(x, y)| x > 6 || map.contains(&(x, y))) {self.tiles = new_positions}
    }

    fn move_down(&mut self, map: &HashSet<(i64, i64)>) -> bool  {
        let new_positions: Vec<(i64, i64)> = self.tiles.iter().map(|&(x, y)| (x, y-1)).collect();
        if new_positions.iter().any(|&(x, y)| y < 1 || map.contains(&(x, y))) {false} else {self.tiles = new_positions; true}
    }

    fn get_nth(n: usize) -> Self {
        let rock_shapes = [
            RockShape::from(vec![(2, 0), (3, 0), (4, 0), (5, 0)]),
            RockShape::from(vec![(3, 0), (2, 1), (3, 1), (4, 1), (3, 2)]),
            RockShape::from(vec![(2, 0), (3, 0), (4, 0), (4, 1), (4, 2)]),
            RockShape::from(vec![(2, 0), (2, 1), (2, 2), (2, 3)]),
            RockShape::from(vec![(2, 0), (3, 0), (2, 1), (3, 1)]),
        ];
        rock_shapes[n].clone()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Direction { LEFT, RIGHT }
