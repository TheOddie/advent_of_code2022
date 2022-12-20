use std::borrow::BorrowMut;
use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::ops::{Add, AddAssign, Sub};
use crate::Tile::{BLOCK, EMPTY};

fn main() {
    init_rock_shapes();
    let input = parse_input(include_str!("../input/day17.txt"));

    println!("{:?}", solve(input));
}

fn solve(directions: Vec<Direction>) -> i32 {

    let mut grid: Vec<[Tile; 7]> = vec![[EMPTY; 7]; 9];
    let mut max = 0;
    let mut dir_num = 0;

    for count in 0..2022 {
        // spawn new rock
        let current_rock = get_rock_shape(count % 5);
        let mut current_rock_pos = Vec2i::new(2, 4 + max);

        // while check_collide
        while check_collide_dir(&grid, &current_rock, &current_rock_pos, Vec2i::new(0, -1))
        {
            // fall 1
            current_rock_pos.y -= 1;

            // draw_grid_with_rock(&grid, &current_rock, &current_rock_pos);

            // push lr
            let dir = directions[dir_num];
            dir_num = (dir_num + 1) % directions.len();
            match dir {
                Direction::LEFT => {
                    if current_rock_pos.x > 0 && check_collide_dir(&grid, &current_rock, &current_rock_pos, Vec2i::new(-1, 0)) {
                        current_rock_pos.x -= 1;
                    }
                }
                Direction::RIGHT => {
                    if current_rock_pos.x + current_rock.extent.x < 7 && check_collide_dir(&grid, &current_rock, &current_rock_pos, Vec2i::new(1, 0)){
                        current_rock_pos.x += 1;
                    }
                }
            }
            // draw_grid_with_rock(&grid, &current_rock, &current_rock_pos);
        }
        // place rock
        for tile in current_rock.tiles {
            let tile_pos = tile + current_rock_pos;
            grid[tile_pos.y as usize][tile_pos.x as usize] = BLOCK;
            max = max.max(tile_pos.y + 1);
        }

        // draw_grid(&grid);
        for _ in grid.len() - 8..max as usize {
            grid.push([EMPTY; 7]);
        }

    }

    max
}

fn check_collide_dir(grid: &Vec<[Tile; 7]>, current_rock: &RockShape, current_rock_pos: &Vec2i, dir: Vec2i) -> bool
{
    let mut result = true;
    for tile in current_rock.tiles.iter()
    {
        let check_tile = *tile + *current_rock_pos + dir;
        if check_tile.y < 0 {return false;}
        result &= grid[check_tile.y as usize][check_tile.x as usize] == EMPTY;
    }
    result
}

fn parse_input(s: &str) -> Vec<Direction>
{
    s.chars().map(|ch| {
        match ch {
            '<' => Direction::LEFT,
            '>' => Direction::RIGHT,
            _ => unreachable!()
        }
    }).collect::<Vec<Direction>>()
}

fn draw_grid(grid: &Vec<[Tile; 7]>)
{
    for line in grid.iter().rev()
    {
        print!("|");
        for x in line {
            let ch = match x {
                EMPTY => {'.'},
                Tile::BLOCK => {'#'},
            };
            print!("{}", ch);
        }
        println!("|");
    }
    println!("+-------+\n");
}

fn draw_grid_with_rock(grid: &Vec<[Tile; 7]>, rock: &RockShape, pos: &Vec2i)
{
    for (j, line) in grid.iter().enumerate().rev()
    {
        print!("|");
        for (i, x) in line.iter().enumerate() {
            let mut ch = match x {
                EMPTY => {'.'},
                Tile::BLOCK => {'#'},
            };
            for tile in rock.tiles.iter() {
                let abs_pos = *tile + *pos;
                if abs_pos.x == i as i32 && abs_pos.y == j as i32 {
                    ch = '@';
                }
            }
            print!("{}", ch);
        }
        println!("|");
    }
    println!("+-------+\n");
}

std::thread_local! {
    static ROCK_SHAPES: RefCell<HashMap<usize, RockShape>> = RefCell::new(HashMap::new());
}

fn init_rock_shapes()
{
    ROCK_SHAPES.with(|hm| {
        let mut hm = hm.borrow_mut();
        hm.insert(
            0,
            RockShape { tiles: vec![
                Vec2i::new(0, 0),
                Vec2i::new(1, 0),
                Vec2i::new(2, 0),
                Vec2i::new(3, 0),
            ],
                extent: Vec2i::new(4, 1)
            }
        );
        hm.insert(
            1,
            RockShape { tiles: vec![
                Vec2i::new(0, 1),
                Vec2i::new(1, 0),
                Vec2i::new(1, 1),
                Vec2i::new(1, 2),
                Vec2i::new(2, 1),
            ],
                extent: Vec2i::new(3, 3)
            }
        );
        hm.insert(
            2,
            RockShape { tiles: vec![
                Vec2i::new(0, 0),
                Vec2i::new(1, 0),
                Vec2i::new(2, 0),
                Vec2i::new(2, 1),
                Vec2i::new(2, 2),
            ],
                extent: Vec2i::new(3, 3)
            }
        );
        hm.insert(
            3,
            RockShape { tiles: vec![
                Vec2i::new(0, 0),
                Vec2i::new(0, 1),
                Vec2i::new(0, 2),
                Vec2i::new(0, 3),
            ],
                extent: Vec2i::new(1, 4)
            }
        );
        hm.insert(
            4,
            RockShape { tiles: vec![
                Vec2i::new(0, 0),
                Vec2i::new(0, 1),
                Vec2i::new(1, 0),
                Vec2i::new(1, 1),
            ],
                extent: Vec2i::new(2, 2)
            }
        );
    });
}

fn get_rock_shape(n: usize) -> RockShape
{
    ROCK_SHAPES.with(|hm| {
        hm.borrow().get(&n).unwrap().clone()
    })
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    LEFT,
    RIGHT,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Tile {
    EMPTY,
    BLOCK,
}

#[derive(Clone, Debug)]
struct RockShape {
    tiles: Vec<Vec2i>,
    extent: Vec2i
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
