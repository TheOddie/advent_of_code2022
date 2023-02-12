use std::collections::{HashMap, HashSet};
use std::slice::Iter;
use std::time::Instant;
use priority_queue::PriorityQueue;

fn main() {
    let start = Instant::now();
    let (initial_grid, target) = parse_input(include_str!("../input/day24.txt"));

    let (graph, size) = generate_graph(&initial_grid);

    println!("{:?}", neighbors(&graph, &(1, 0, 0), &size));
    println!("{:?}", target);
    println!("{:?}", a_star(&graph, (1, 0, 0), target, size));
    println!("Done in {}ms", start.elapsed().as_millis());
}

fn dijkstra(graph: &HashSet<(i16, i16, i16)>, start: (i16, i16, i16), target: (i16, i16), size: (i16, i16, i16)) -> i16 {
    let mut dist = HashMap::new();
    let mut prev: HashMap<(i16, i16, i16), Option<(i16, i16, i16)>> = HashMap::new();
    let mut q = PriorityQueue::new();

    for vertex in graph {
        dist.insert(vertex.clone(), 32_000);
        prev.insert(vertex.clone(), None);
        q.push(vertex.clone(), i16::MIN);
    }
    dist.insert(start.clone(), 0);
    q.change_priority(&start, 0);

    while q.len() != 0 {
        let u = q.pop().unwrap().0;

        let mq = q.clone();
        for v in neighbors(graph, &u, &size)
            .iter()
            .filter(|&v| mq.get_priority(v).is_some())
        {
            let alt = dist.get(&u).unwrap() + 1;
            if alt < *dist.get(v).unwrap() {
                dist.insert(*v, alt);
                q.change_priority(v, -alt);
                prev.insert(*v, Some(u));
            }
        }
        if q.len() % 100 == 0 {
            println!("{}", q.len());
        }
    }

    let result = dist.iter()
        .filter(|&(&v, _)| v.0 == target.0 && v.1 == target.1)
        .min_by(|&(_, &a), &(_, b)| a.cmp(b))
        .unwrap();

    println!("{:?}", result);

    *result.1
}

fn a_star(graph: &HashSet<(i16, i16, i16)>, start: (i16, i16, i16), target: (i16, i16), size: (i16, i16, i16)) -> usize {
    let h = |v: (i16, i16, i16), t: (i16, i16)| {(v.0.abs_diff(t.0) + v.1.abs_diff(t.1)) as i16};

    let mut open_set = PriorityQueue::new();
    open_set.push(start.clone(), 0);
    let mut came_from = HashMap::new();

    let mut g_score = HashMap::new();
    g_score.insert(start.clone(), 0);

    let mut f_score = HashMap::new();
    f_score.insert(start.clone(), h(start.clone(), target.clone()));

    while open_set.len() != 0 {
        let current = open_set.pop().unwrap().0;
        if current.0 == target.0 && current.1 == target.1 {
            return reconstruct_path(&came_from, current).len() - 1;
        }

        for neighbor in neighbors(graph, &current, &size) {
            let tentative_g_score = *g_score.get(&current).unwrap_or(&32_000) + 1;
            if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&32_000) {
                came_from.insert(neighbor.clone(), current.clone());
                g_score.insert(neighbor.clone(), tentative_g_score);
                let new_f_score = tentative_g_score + h(neighbor.clone(), target.clone());
                f_score.insert(neighbor.clone(), new_f_score);
                if open_set.change_priority(&neighbor, -new_f_score).is_none() {
                    open_set.push(neighbor, -new_f_score);
                }
            }
        }
    }

    usize::MAX
}

fn reconstruct_path(came_from: &HashMap<(i16, i16, i16), (i16, i16, i16)>, mut current: (i16, i16, i16)) -> Vec<(i16, i16, i16)> {
    let mut total_path = vec![current.clone()];
    while came_from.contains_key(&current) {
        current = came_from.get(&current).unwrap().clone();
        total_path.push(current.clone());
    }
    total_path
}

fn generate_graph(initial_grid: &Vec<Vec<Tile>>) -> (HashSet<(i16, i16, i16)>, (i16, i16, i16))
{
    let mut extent = (
        initial_grid[0].len(),
        initial_grid.len(),
        0
    );
    extent.2 = lcm(extent.0 as i16 - 2, extent.1 as i16 - 2) as usize;

    let mut grids = vec![initial_grid.clone()];
    for i in 1..extent.2 {
        grids.push(step_grid(&grids[i-1]));
    }

    // for g in &grids {
    //     println!();
    //     draw_grid(g);
    // }

    let mut result = HashSet::new();

    for (z, plane) in grids.iter().enumerate() {
        for (y, row) in plane.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if tile == &Tile::Empty {
                    result.insert((x as i16, y as i16, z as i16));
                }
            }
        }
    }

    (result, (extent.0 as i16, extent.1 as i16, extent.2 as i16))
}

fn neighbors(graph: &HashSet<(i16, i16, i16)>, vertex: &(i16, i16, i16), size: &(i16, i16, i16)) -> Vec<(i16, i16, i16)>
{
    let frame = (vertex.2 + 1) % size.2;
    Move::iterator()
        .map(|m| m.to_dir())
        .map(|(dx, dy)| (vertex.0 + dx, vertex.1 + dy, frame))
        .filter(|v| graph.contains(v))
        .collect::<Vec<_>>()
}

fn parse_input(input: &str) -> (Vec<Vec<Tile>>, (i16, i16))
{
    let mut result = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for ch in line.chars() {
            row.push(Tile::from(ch).unwrap());
        }
        result.push(row);
    }
    let target_y = input.lines().enumerate().last().unwrap().0 as i16;
    let target_x =  (input.lines().enumerate().last().unwrap().1.chars().enumerate().last().unwrap().0-1) as i16;
    (result, (target_x, target_y))
}

fn step_grid(grid: &Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    let my = grid.len();
    let mx = grid[0].len();
    let mut result = vec![vec![Tile::Empty; mx]; my];
    for (y, row) in grid.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            match tile {
                Tile::Empty => {}
                Tile::Wall => {result[y][x] = Tile::Wall}
                Tile::Blizzards(blizzards) => {
                    for blizzard in blizzards.iter() {
                        let dir = blizzard.to_dir();
                        let new_pos = (
                            modulo(x as i16 + dir.0 - 1, mx as i16 - 2) as usize + 1,
                            modulo(y as i16 + dir.1 - 1, my as i16 - 2) as usize + 1
                        );
                        result[new_pos.1][new_pos.0].add_blizzard(*blizzard).unwrap();
                    }
                }
            }
        }
    }
    result
}

fn modulo(a: i16, b: i16) -> i16 {
    ((a % b) + b) % b
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Blizzard {
    Up, Down, Left, Right
}

impl Blizzard {
    fn from(ch: char) -> Result<Self, String> {
        match ch {
            '^' => Ok(Blizzard::Up),
            'v' => Ok(Blizzard::Down),
            '<' => Ok(Blizzard::Left),
            '>' => Ok(Blizzard::Right),
            _ => Err(String::from("ERROR: Failed to parse."))
        }
    }

    fn to_dir(&self) -> (i16, i16) {
        match self {
            Blizzard::Up => (0, -1),
            Blizzard::Down => (0, 1),
            Blizzard::Left => (-1, 0),
            Blizzard::Right => (1, 0),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
enum Tile {
    Empty,
    Wall,
    Blizzards(Vec<Blizzard>)
}

impl Tile {
    fn from(ch: char) -> Result<Self, String> {
        match ch {
            '.' => Ok(Tile::Empty),
            '#' => Ok(Tile::Wall),
            _ => {
                match Blizzard::from(ch) {
                    Ok(b) => Ok(Tile::Blizzards(vec![b])),
                    Err(e) => Err(e),
                }
            }
        }
    }

    fn add_blizzard(&mut self, blizzard: Blizzard) -> Result<(), String> {
        match self {
            Tile::Empty => {*self = Tile::Blizzards(vec![blizzard]); Ok(())},
            Tile::Wall => Err(String::from("ERROR: Tried to place a blizzard on a wall!")),
            Tile::Blizzards(b) => {b.push(blizzard); Ok(())}
        }
    }
}

enum Move { Up, Down, Left, Right, Wait }

impl Move {
    fn to_dir(&self) -> (i16, i16) {
        match self {
            Move::Up => (0, -1),
            Move::Down => (0, 1),
            Move::Left => (-1, 0),
            Move::Right => (1, 0),
            Move::Wait => (0, 0),
        }
    }

    fn iterator() -> Iter<'static, Move> {
        static MOVES: [Move; 5] = [
            Move::Up,
            Move::Down,
            Move::Left,
            Move::Right,
            Move::Wait,
        ];
        MOVES.iter()
    }
}

fn lcm(a : i16, b : i16) -> i16
{
    (a * b) / gcd(a, b)
}

fn gcd(a : i16, b : i16) -> i16
{
    if b == 0 { a } else { gcd(b, a % b) }
}
