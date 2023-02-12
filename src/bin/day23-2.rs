use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::Add;
use std::slice::Iter;
use std::time::Instant;
use crate::Direction::*;

fn main() {
    let start = Instant::now();
    let mut elves = parse_input(include_str!("../input/day23.txt"));
    let mut direction_list = initial_proposed_directions();

    print_elves(&elves);

    let mut finished = false;
    let mut count = 0;

    while !finished {
        // println!("\n");
        let (new_elves, new_finished) = step(&elves, &direction_list);
        elves = new_elves;
        finished = new_finished;
        let dir = direction_list.pop_front().unwrap();
        direction_list.push_back(dir);
        // print_elves(&elves);
        count += 1;
    }

    // print_elves(&elves);
    println!("{:?}", count);
    println!("Done in {}ms", start.elapsed().as_millis());
}

fn step(elves: &HashSet<Pos>, proposed_directions: &VecDeque<(Direction, [Direction; 3])>) -> (HashSet<Pos>, bool)
{
    let mut movement = HashMap::new();
    let mut no_move = HashSet::new();

    // Step One
    'elf_loop: for &elf in elves {
        if Direction::iterator().any(|&dir| elves.contains(&(elf + dir.to_pos()))) {
            for &(move_dir, dir_list) in proposed_directions {
                if dir_list.iter().all(|dir| {
                    !elves.contains(&(elf + dir.to_pos()))
                }) {
                    if let Some(prev_elf) = movement.insert(elf + move_dir.to_pos(), elf) {
                        no_move.insert(elf);
                        no_move.insert(prev_elf);
                    }
                    continue 'elf_loop;
                }
            }
        }
        no_move.insert(elf);
    }

    let mut result = HashSet::new();

    for &elf in &no_move {
        assert!(result.insert(elf));
    }
    for (&new_pos, &elf) in &movement {
        if !no_move.contains(&elf) {
            result.insert(new_pos);
        }
    }
    assert_eq!(elves.len(), result.len());

    (result, movement.is_empty())
}

fn parse_input(s: &str) -> HashSet<Pos>
{
    let mut elves = HashSet::new();
    for (y, line) in s.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                elves.insert(Pos::new(x as i32, y as i32));
            }
        }
    }
    elves
}

fn initial_proposed_directions() -> VecDeque<(Direction, [Direction; 3])> {
    let mut result = VecDeque::new();
    result.push_back((N, [N, NE, NW]));
    result.push_back((S, [S, SE, SW]));
    result.push_back((W, [W, NW, SW]));
    result.push_back((E, [E, NE, SE]));
    result
}

fn print_elves(elves: &HashSet<Pos>) {
    let mut min = Pos::new(i32::MAX, i32::MAX);
    let mut max = Pos::new(i32::MIN, i32::MIN);

    for &elf in elves {
        min.x = min.x.min(elf.x);
        min.y = min.y.min(elf.y);
        max.x = max.x.max(elf.x);
        max.y = max.y.max(elf.y);
    }

    for y in min.y..=max.y {
        for x in min.x..=max.x {
            let ch = match elves.contains(&Pos::new(x, y)) {
                true => {'#'}
                false => {'.'}
            };
            print!(" {}", ch);
        }
        println!();
    }
}

fn count_empty_tiles(elves: &HashSet<Pos>) -> i32 {
    let mut min = Pos::new(i32::MAX, i32::MAX);
    let mut max = Pos::new(i32::MIN, i32::MIN);

    for &elf in elves {
        min.x = min.x.min(elf.x);
        min.y = min.y.min(elf.y);
        max.x = max.x.max(elf.x);
        max.y = max.y.max(elf.y);
    }

    let size = (max.x - min.x + 1) * (max.y - min.y + 1);
    // println!("size: {}, len: {}, count2: {}, real: {}", size, elves.len(), size - elves.len() as i32, count);

    size - elves.len() as i32
}

#[derive(Copy, Clone, Hash, Debug, Eq, PartialEq)]
struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Self {
        Pos { x, y }
    }
}

impl Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        Pos { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

#[derive(Copy, Clone, Hash, Debug, Eq, PartialEq)]
enum Direction {
    N, NE, NW, S, SE, SW, W, E
}

impl Direction {
    fn to_pos(&self) -> Pos {
        match self {
            N => {Pos::new(0, -1)}
            NE => {Pos::new(1, -1)}
            NW => {Pos::new(-1, -1)}
            S => {Pos::new(0, 1)}
            SE => {Pos::new(1, 1)}
            SW => {Pos::new(-1, 1)}
            W => {Pos::new(-1, 0)}
            E => {Pos::new(1, 0)}
        }
    }

    fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 8] = [N, NE, E, SE, S, SW, W, NW];
        DIRECTIONS.iter()
    }
}
