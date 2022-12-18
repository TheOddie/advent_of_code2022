use std::ops::{Add, AddAssign, Sub};
use scanf::sscanf;

fn main() {
    let input = parse_input(include_str!("../input/day18.txt"));

    println!("{:?}", solve(input));
}

fn solve(cubes: Vec<Vec3i>) -> i32
{
    let mut max = Vec3i::new(0, 0, 0);
    for cube in &cubes {
        max.x = max.x.max(cube.x);
        max.y = max.y.max(cube.y);
        max.z = max.z.max(cube.z);
    }

    let mut grid = vec![vec![vec![false; max.x as usize + 2]; max.y as usize + 2]; max.z as usize + 2];

    for cube in &cubes {
        grid[cube.z as usize][cube.y as usize][cube.x as usize] = true;
    }

    let mut count = 0;
    for cube in &cubes {
        count += empty_adjacents(&cube, &grid);
    }

    count
}

fn empty_adjacents(i: &Vec3i, grid: &Vec<Vec<Vec<bool>>>) -> i32
{
    const DIRECTIONS: [Vec3i; 6] = [
        Vec3i { x: 1, y: 0, z: 0 },
        Vec3i { x: -1, y: 0, z: 0 },
        Vec3i { x: 0, y: 1, z: 0 },
        Vec3i { x: 0, y: -1, z: 0 },
        Vec3i { x: 0, y: 0, z: 1 },
        Vec3i { x: 0, y: 0, z: -1 }
    ];
    let mut count = 0;

    for dir in &DIRECTIONS {
        let m = *i + *dir;
        if m.x < 0 || m.y < 0 || m.z < 0 {
            count += 1;
            continue;
        }
        if !grid[m.z as usize][m.y as usize][m.x as usize] {
            count += 1;
        }
    }
    count
}

fn parse_input(s: &str) -> Vec<Vec3i>
{
    let mut cubes = Vec::new();
    for cube in s.lines() {
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut z: i32 = 0;
        sscanf!(cube, "{},{},{}", x, y, z).unwrap();
        cubes.push(Vec3i::new(x, y, z));
    }
    cubes
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Vec3i {
    x: i32,
    y: i32,
    z: i32
}

impl Vec3i {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Vec3i { x, y, z }
    }
}

impl Add for Vec3i {
    type Output = Vec3i;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3i { x: self.x + rhs.x,
                y: self.y + rhs.y,
                z: self.z + rhs.z}
    }
}

impl Sub for Vec3i {
    type Output = Vec3i;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3i { x: self.x - rhs.x,
                y: self.y - rhs.y,
                z: self.z - rhs.z }
    }
}

impl AddAssign for Vec3i {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
