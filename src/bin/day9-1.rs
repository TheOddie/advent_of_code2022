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
    let mut head: (i32, i32) = (0, 0);
    let mut tail: (i32, i32) = (0, 0);
    let mut visited_nodes: HashSet<(i32, i32)> = HashSet::new();

    for movement in input {
        let (direction, count) = movement.unpack();
        for _ in 0..count {
            head.0 += direction.0;
            head.1 += direction.1;

            tail = update_tail(&head, &tail);
            visited_nodes.insert(tail.clone());
        }
    }

    println!("{:?}", visited_nodes);

    visited_nodes.len()
}

fn update_tail(head: &(i32, i32), tail: &(i32, i32)) -> (i32, i32)
{
    let distance = (head.0 - tail.0, head.1 - tail.1);
    if -1 <= distance.0 && distance.0 <= 1 && -1 <= distance.1 && distance.1 <= 1 {
        return *tail;
    }
    return if distance.0.abs() > distance.1.abs() { // x > y
        (tail.0 + (distance.0 / 2), tail.1 + distance.1)
    } else {
        (tail.0 + distance.0, tail.1 + (distance.1 / 2))
    }
}
