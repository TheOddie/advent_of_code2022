use std::ops::{Add, AddAssign, Sub};
use std::time::Instant;
use scanf::sscanf;

fn main() {
    let start = Instant::now();
    let (input, max, min) = parse_input(include_str!("../input/day14.txt"));

    let result = solve(input, (max, min));
    println!("{:?} in {}ms", result, start.elapsed().as_millis());
}

fn solve(rock_lines: Vec<Vec<Vec2i>>, max_min: (Vec2i, Vec2i)) -> u32 {
    let mut grid = vec![vec![Tile::AIR; (2 * max_min.0.x) as usize]; (max_min.0.y + 3) as usize];
    let source = Vec2i::new(500, 0);

    grid[source.y as usize][source.x as usize] = Tile::SOURCE;

    for rock_line in rock_lines {
        for i in 0..(rock_line.len() - 1) {
            let dist = rock_line[i+1] - rock_line[i];
            let step = Vec2i::new(dist.x.signum(), dist.y.signum());
            let mut p = rock_line[i];
            let lim = rock_line[i+1];
            while p != lim {
                grid[p.y as usize][p.x as usize] = Tile::ROCK;
                p += step;
            }
            grid[lim.y as usize][lim.x as usize] = Tile::ROCK;
        }
    }
    let len = grid[0].len();
    for i in 0..len {
        let y_len = grid.len();
        grid[y_len - 1 as usize][i] = Tile::ROCK;
    }


    let mut counter = 0;
    loop {
        let new_sand_position = new_sand(&grid, source);
        match new_sand_position {
            None => {break;}
            Some(p) => {
                grid[p.y as usize][p.x as usize] = Tile::SAND;
            }
        }
        counter += 1;
        if grid[source.y as usize][source.x as usize] != Tile::SOURCE {
            break;
        }
    }

    print_map(&grid);

    counter
}

fn parse_input(s: &str) -> (Vec<Vec<Vec2i>>, Vec2i, Vec2i)
{
    let mut rock_lines = Vec::new();
    let mut max = Vec2i::new(i32::MIN, i32::MIN);
    let mut min = Vec2i::new(i32::MAX, i32::MAX);

    for line in s.lines() {

        let mut rock_line = Vec::new();
        for token in line.split(" -> ") {
            let mut x: i32 = 0;
            let mut y: i32 = 0;
            sscanf!(token, "{},{}", x, y).unwrap();
            rock_line.push(Vec2i::new(x, y));
            max.x = max.x.max(x);
            max.y = max.y.max(y);
            min.x = min.x.min(x);
            min.y = min.y.min(y);
        }
        rock_lines.push(rock_line);
    }

    (rock_lines, max, min)
}

fn print_map(grid: &Vec<Vec<Tile>>)
{
    for row in grid {
        for tile in row {
            print!("{} ", tile.to_char());
        }
        println!();
    }
}

fn new_sand(grid: &Vec<Vec<Tile>>, start: Vec2i) -> Option<Vec2i>
{
    let steps: [Vec2i; 3] = [Vec2i::new(0, 1), Vec2i::new(-1, 1), Vec2i::new(1, 1)];
    let mut sand_pos = start;

    loop {  // I tried using break, but it wasn't working so I had to make this monstrosity instead.
        let mut flag = false;
        for i in 0..steps.len() {
            if flag {
                continue;
            }
            let step = steps[i];
            let next_pos = sand_pos + step;
            if next_pos.y >= grid.len() as i32 {
                return None;
            }
            if grid[next_pos.y as usize][next_pos.x as usize] == Tile::AIR {
                sand_pos += step;
                flag = true;
            }
        }
        if !flag {
            return Some(sand_pos);
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Tile {
    AIR,
    ROCK,
    SAND,
    SOURCE
}

impl Tile {
    fn to_char(&self) -> char {
        match self {
            Tile::AIR => {'.'}
            Tile::ROCK => {'#'}
            Tile::SAND => {'o'}
            Tile::SOURCE => {'+'}
        }
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
struct Vec2i {
    x: i32,
    y: i32
}

impl Vec2i {
    fn new(x: i32, y: i32) -> Self {
        Vec2i { x, y }
    }
}

impl Add for Vec2i {
    type Output = Vec2i;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2i { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl Sub for Vec2i {
    type Output = Vec2i;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec2i { x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl AddAssign for Vec2i {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
