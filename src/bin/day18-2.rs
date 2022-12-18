use std::collections::HashSet;
use std::ops::{Add, AddAssign, Sub};
use scanf::sscanf;

const DIRECTIONS: [Vec3i; 6] = [
    Vec3i { x: 1, y: 0, z: 0 },
    Vec3i { x: -1, y: 0, z: 0 },
    Vec3i { x: 0, y: 1, z: 0 },
    Vec3i { x: 0, y: -1, z: 0 },
    Vec3i { x: 0, y: 0, z: 1 },
    Vec3i { x: 0, y: 0, z: -1 }
];

fn main() {
    let input = parse_input(include_str!("../input/day18.txt"));

    println!("{:?}", solve(input));
}

fn solve(cubes: HashSet<Vec3i>) -> i32
{
    let mut max = Vec3i::new(i32::MIN, i32::MIN, i32::MIN);
    let mut min = Vec3i::new(i32::MAX, i32::MAX, i32::MAX);
    for cube in &cubes {
        max.x = max.x.max(cube.x);
        max.y = max.y.max(cube.y);
        max.z = max.z.max(cube.z);
        min.x = min.x.min(cube.x);
        min.y = min.y.min(cube.y);
        min.z = min.z.min(cube.z);
    }
    min += Vec3i::new(-1, -1, -1);
    max += Vec3i::new(2, 2, 2);

    let mut to_fill = vec![min];
    let mut filled = HashSet::new();

    while to_fill.len() != 0 {
        let w = to_fill.pop().unwrap();
        if filled.contains(&w) || cubes.contains(&w) {
            continue;
        }
        filled.insert(w.clone());
        for dir in &DIRECTIONS {
            let t = w + *dir;
            if in_range(&t, &min, &max) {
                to_fill.push(t);
            }
        }
    }

    let mut count = 0;
    for cube in &cubes {
        for dir in &DIRECTIONS {
            let w = *cube + *dir;
            if filled.contains(&w) {
                count += 1
            }
        }
    }

    count
}

fn in_range(v: &Vec3i, min: &Vec3i, max: &Vec3i) -> bool {
    min.x <= v.x && v.x < max.x && min.y <= v.y && v.y < max.y && min.z <= v.z && v.z < max.z
}

fn parse_input(s: &str) -> HashSet<Vec3i>
{
    let mut cubes = HashSet::new();
    for cube in s.lines() {
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut z: i32 = 0;
        sscanf!(cube, "{},{},{}", x, y, z).unwrap();
        cubes.insert(Vec3i::new(x + 1, y + 1, z + 1));
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
