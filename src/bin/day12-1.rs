use std::collections::HashSet;
use std::ops::Add;
use std::time::Instant;
use crate::Directions::{DOWN, LEFT, RIGHT, UP};

fn main() {
    let (graph, start, goal) = parse_input(include_str!("../input/day12.txt"));

    let start_time = Instant::now();
    let (cost, path) = shortest_path_2(&graph, start, goal);
    println!("Done in {}ms", start_time.elapsed().as_millis());

    println!("{:?}, {:?}", cost, path);
}

fn shortest_path_2(graph: &Vec<Vec<Vec<Edge>>>, initial_position: Pos, destination: Pos) -> (i32, Vec<Pos>) {

    let mut unvisited = HashSet::new();
    for y in 0..graph.len() {
        for x in 0..graph[y].len() {
            unvisited.insert(Pos { x: x as i32, y: y as i32 });
        }
    }
    let mut dist: Vec<Vec<(u32, Option<Pos>)>> = vec![vec![(u32::MAX, None); graph[0].len()]; graph.len()];

    dist[initial_position.y as usize][initial_position.x as usize] = (0, None);

    while unvisited.contains(&destination) {
        // wtf, iterating over a hashset is super slow
        let current_node = unvisited.iter()
            .min_by(|a, b| dist[a.y as usize][a.x as usize].0.cmp(&dist[b.y as usize][b.x as usize].0))
            .unwrap().clone();

        for neighbor in &graph[current_node.y as usize][current_node.x as usize] {
            if !unvisited.contains(&neighbor.node) {continue}
            let new_distance = dist[current_node.y as usize][current_node.x as usize].0 + neighbor.cost as u32;
            if dist[neighbor.node.y as usize][neighbor.node.x as usize].0 > new_distance {
                dist[neighbor.node.y as usize][neighbor.node.x as usize] = (new_distance, Some(current_node));
            }
        }
        unvisited.remove(&current_node);
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

fn parse_input(s: &str) -> (Vec<Vec<Vec<Edge>>>, Pos, Pos)
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

    (graph, start_node, end_node)
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
