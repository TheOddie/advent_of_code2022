use std::collections::{HashMap, HashSet};
use std::ops::{Add, AddAssign, Sub};
use std::time::Instant;
use scanf::sscanf;

fn main() {
    let start = Instant::now();
    let (sensors, beacons) = parse_input(include_str!("../input/day15.txt"));

    let result = solve_line(sensors, beacons, 2000000);

    println!("{:?} found in {}us", result, start.elapsed().as_micros());
}

fn solve_line(sensors: HashMap<Vec2i, (Vec2i, i32)>, beacons: HashSet<Vec2i>, line: i32) -> i32 {

    let mut segments = Vec::new();

    for (sensor, (_beacon, distance)) in sensors {
        let y_diff = (sensor.y - line).abs();
        let segment_length = ((2 * distance + 1) - (2 * y_diff)).max(0);
        if segment_length == 0 {continue;}
        let x_diff = (segment_length - 1) / 2;
        let lower = sensor.x - x_diff;
        let upper = sensor.x + x_diff;
        segments.push((lower, upper));
    }


    segments.sort_by(|(a, _), (b, _)| a.cmp(b));
    let mut beacons_on_line = beacons.iter().filter(|x| x.y == line).collect::<Vec<&Vec2i>>();
    // println!("{:?}", segments);

    let mut max = i32::MIN;
    let mut count = 0;

    for (lower, upper) in segments {

        let mut overlap = 0;
        if max >= lower {
            overlap = max - lower + 1;
        }
        let mut length = (upper - lower + 1 - overlap).max(0);

        for (index, beacon) in beacons_on_line.clone().iter().enumerate() {
            if lower <= beacon.x && beacon.x <= upper {
                length -= 1;
                beacons_on_line.remove(index);
                // println!("Found beacon on line at location {}", beacon.x);
            }
        }

        count += length;
        max = upper.max(max);
        // println!("Added {} to length", length);
    }

    count
}

fn parse_input(s: &str) -> (HashMap<Vec2i, (Vec2i, i32)>, HashSet<Vec2i>)
{
    let mut sensors: HashMap<Vec2i, (Vec2i, i32)> = HashMap::new();
    let mut beacons: HashSet<Vec2i> = HashSet::new();

    for line in s.lines() {
        let mut sx: i32 = 0;
        let mut sy: i32 = 0;
        let mut bx: i32 = 0;
        let mut by: i32 = 0;
        sscanf!(line, "Sensor at x={}, y={}: closest beacon is at x={}, y={}", sx, sy, bx, by).unwrap();
        let sensor = Vec2i::new(sx, sy);
        let beacon = Vec2i::new(bx, by);

        let diff = sensor - beacon;
        let dist = diff.x.abs() + diff.y.abs();

        sensors.insert(sensor, (beacon.clone(), dist));
        beacons.insert(beacon);
    }

    (sensors, beacons)
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
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
