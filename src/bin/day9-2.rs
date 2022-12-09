use std::collections::HashSet;
use scanf::sscanf;

fn main() {
    let input = parse_input(include_str!("../input/day9.txt"));

    println!("{:?}", solve(input));
}

#[derive(Debug, Copy, Clone)]
enum Direction
{
    U(u32), D(u32), L(u32), R(u32)
}

impl Direction {
    fn unpack(&self) -> ((i32, i32), u32)
    {
        match self {
            Direction::U(count) => {((0, 1), *count)}
            Direction::D(count) => {((0, -1), *count)}
            Direction::L(count) => {((-1, 0), *count)}
            Direction::R(count) => {((1, 0), *count)}
        }
    }
}

fn parse_input(s: &str) -> Vec<Direction>
{
    s.lines().map(
        |line| {
            let mut direction: String = String::new();
            let mut count: u32 = 0;

            sscanf!(line, "{} {}", direction, count).unwrap();

            match direction.as_str() {
                "U" => Direction::U(count),
                "D" => Direction::D(count),
                "L" => Direction::L(count),
                "R" => Direction::R(count),
                _ => unreachable!(),
            }
        }
    ).collect()
}

fn solve(input: Vec<Direction>) -> usize
{
    let mut rope = vec![(0, 0); 10];
    let mut visited_nodes: HashSet<(i32, i32)> = HashSet::new();

    for movement in input {
        let (direction, count) = movement.unpack();
        for _ in 0..count {
            rope[0].0 += direction.0;
            rope[0].1 += direction.1;
            for i in 0..rope.len()-1 {
                rope[i+1] = update_tail(&rope[i], &rope[i+1]);
            }
            println!("{:?}", rope.last().unwrap());
            visited_nodes.insert(rope.last().unwrap().clone());
        }
    }

    // print_grid(&visited_nodes);

    visited_nodes.len()
}

fn update_tail(head: &(i32, i32), tail: &(i32, i32)) -> (i32, i32)
{
    let distance = (head.0 - tail.0, head.1 - tail.1);
    if -1 <= distance.0 && distance.0 <= 1 && -1 <= distance.1 && distance.1 <= 1 {
        return *tail;
    }
    return (tail.0 + distance.0.signum(), tail.1 + distance.1.signum());
}

fn print_grid(visited_nodes: &HashSet<(i32, i32)>)
{
    for y in (-20..20).rev() {
        for x in -20..20 {
            print!("{}", if visited_nodes.contains(&(x, y)) {'#'} else {'.'});
        }
        println!();
    }
}
