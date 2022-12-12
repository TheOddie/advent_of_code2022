use std::collections::HashSet;
use std::ops::Add;
use std::time::Instant;
use priority_queue::PriorityQueue;
use crate::Directions::{DOWN, LEFT, RIGHT, UP};

fn main() {
    let (grid, graph, _start, goal) = parse_input(include_str!("../input/day12.txt"));

    let mut a_positions = Vec::new();
    for (y, row) in grid.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == 'a' {
                a_positions.push(Pos{ x: x as i32, y: y as i32 });
            }
        }
    }

    let start_time = Instant::now();
    let min = a_positions.iter()
        .map(|pos| shortest_path(&graph, *pos, goal).0)
        .min()
        .unwrap();
    println!("Done in {}ms", start_time.elapsed().as_millis()); // took me 8 and a half seconds, very slow :(

    println!("{:?}", min);
}

fn shortest_path(graph: &Vec<Vec<Vec<Edge>>>, initial_position: Pos, destination: Pos) -> (i32, Vec<Pos>) {

    let mut unvisited = PriorityQueue::new();
    let mut visited_set = HashSet::new();
    for y in 0..graph.len() {
        for x in 0..graph[y].len() {
            unvisited.push(Pos { x: x as i32, y: y as i32 }, 0);
        }
    }
    let mut dist: Vec<Vec<(u32, Option<Pos>)>> = vec![vec![(u32::MAX, None); graph[0].len()]; graph.len()];

    dist[initial_position.y as usize][initial_position.x as usize] = (0, None);
    unvisited.change_priority(&initial_position, u32::MAX);

    loop {

        let current_node = match unvisited.pop() {
            Some(s) => s.0,
            None => break
        };

        if current_node == destination {
            break;
        }

        if dist[current_node.y as usize][current_node.x as usize].0 == u32::MAX {
            return (i32::MAX, Vec::new());
        }

        for neighbor in &graph[current_node.y as usize][current_node.x as usize] {
            if visited_set.contains(&neighbor.node) {continue}
            let new_distance = dist[current_node.y as usize][current_node.x as usize].0 + neighbor.cost as u32;
            if dist[neighbor.node.y as usize][neighbor.node.x as usize].0 > new_distance {
                dist[neighbor.node.y as usize][neighbor.node.x as usize] = (new_distance, Some(current_node));
                unvisited.change_priority(&neighbor.node, u32::MAX - new_distance);
            }
        }

        visited_set.insert(current_node.clone());

    }

    let mut output = Vec::new();
    let mut current_node = destination;
    loop {
        match dist[current_node.y as usize][current_node.x as usize].1 {
            None => break,
            Some(s) => {
                output.push(s);
                current_node = s;
            }
        }
    }
    output.reverse();
    return (output.len() as i32, output);
}

fn parse_input(s: &str) -> (Vec<Vec<char>>, Vec<Vec<Vec<Edge>>>, Pos, Pos)
{
    let mut grid: Vec<Vec<char>> = s.lines().map(|line| line.chars().collect::<Vec<char>>()).collect();
    let mut start_node = Pos { x: 0, y: 0 };
    let mut end_node = Pos { x: 0, y: 0 };

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let this_tile = Pos { x: x as i32, y: y as i32 };
            if grid[y][x] == 'S' {
                start_node = this_tile.clone();
                grid[y][x] = 'a';
            }
            if grid[y][x] == 'E' {
                end_node = this_tile.clone();
                grid[y][x] = 'z';
            }
        }
    }

    let mut graph = s.lines().map(|line| line.chars().map(|_| Vec::new()).collect::<Vec<Vec<Edge>>>()).collect::<Vec<Vec<Vec<Edge>>>>();

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let this_tile = Pos { x: x as i32, y: y as i32 };
            for direction in Directions::iter() {
                let other_tile = this_tile + direction.get_direction();
                if other_tile.x < 0 || other_tile.x >= grid[y].len() as i32 { continue; }
                if other_tile.y < 0 || other_tile.y >= grid.len() as i32 { continue; }
                let dist = grid[other_tile.y as usize][other_tile.x as usize] as i32 - grid[y][x] as i32;
                if dist <= 1 {
                    graph[y][x].push(Edge { node: other_tile, cost: 1 })
                }
            }
        }
    }

    (grid, graph, start_node, end_node)
}

#[derive(Debug)]
struct Edge {
    node: Pos,
    cost: i32,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Pos {
    x: i32,
    y: i32,
}

impl Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        Pos { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

#[derive(Copy, Clone, Debug)]
enum Directions {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Directions {
    fn get_direction(&self) -> Pos {
        match self {
            UP => {Pos { x: 0, y: 1 }}
            DOWN => {Pos { x: 0, y: -1 }}
            LEFT => {Pos { x: -1, y: 0 }}
            RIGHT => {Pos { x: 1, y: 0 }}
        }
    }

    fn iter() -> impl Iterator<Item = Directions> {
        [UP, DOWN, LEFT, RIGHT].iter().copied()
    }
}
