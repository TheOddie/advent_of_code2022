use crate::Instruction::{Forward, Left, Right};
use crate::Tile::{Block, Space};

fn main() {
    let input = parse_input(include_str!("../input/day22.txt"));

    // for line in &input.0 {
    //     for &ch in line {
    //         let c = match ch {
    //             None => {'@'}
    //             Some(tile) => {
    //                 match tile {
    //                     Space => {'.'}
    //                     Block => {'#'}
    //                 }
    //             }
    //         };
    //         print!("{}", c);
    //     }
    //     println!();
    // }
    // println!("{:?}", input.1);

    let result = solve(input.0, input.1);
    println!("{:?}", result);
    println!("{:?}", 1000 * result.0.1 + 4 * result.0.0 + result.1);
}

fn solve(map: Vec<Vec<Option<Tile>>>, instructions: Vec<Instruction>) -> ((usize, usize), usize) {
    let mut pos = (0, 0);
    let extent = (
        map.iter().map(|line| line.len()).max().unwrap(),
        map.len(),
    );
    while map[pos.1][pos.0].is_none() || map[pos.1][pos.0].unwrap() == Block {
        pos.0 += 1;
    }
    let mut facing = 0;
    for instruction in instructions {
        match instruction {
            Forward(count) => {
                let dir = get_dir(facing);
                for _ in 0..count {
                    let mut next_pos = (
                        modulo(pos.0 as i32 + dir.0, extent.0 as i32) as usize,
                        modulo(pos.1 as i32 + dir.1, extent.1 as i32) as usize
                        );
                    match map[next_pos.1][next_pos.0] {
                        None => {
                            next_pos = wrap(&map, extent, dir, next_pos);
                            match map[next_pos.1][next_pos.0].unwrap() {
                                Space => {pos = next_pos;}
                                Block => {break;}
                            }
                        }
                        Some(tile) => {
                            match tile {
                                Space => {pos = next_pos;}
                                Block => {break;}
                            }
                        }
                    }
                }
            }
            Right => {facing = (facing + 1) % 4}
            Left => {facing = (facing + 3) % 4}
        }
    }
    ((pos.0 + 1, pos.1 + 1), facing)
}

fn wrap(map: &Vec<Vec<Option<Tile>>>, extent: (usize, usize), dir: (i32, i32), pos: (usize, usize)) -> (usize, usize) {
    let dir = (-dir.0, -dir.1);
    let mut current_pos = pos;
    let mut flag = false;
    loop {
        let next_pos = (
            modulo(current_pos.0 as i32 + dir.0, extent.0 as i32) as usize,
            modulo(current_pos.1 as i32 + dir.1, extent.1 as i32) as usize
        );
        match map.get(next_pos.1) {
            None => {if flag {return current_pos} else {current_pos = next_pos}}
            Some(line) => {
                match line.get(next_pos.0) {
                    None => {if flag {return current_pos} else {current_pos = next_pos}}
                    Some(tile) => {
                        match tile {
                            None => {if flag {return current_pos} else {current_pos = next_pos}}
                            Some(_) => {flag = true; current_pos = next_pos}
                        }
                    }
                }
            }
        }
    }
}

fn get_dir(facing: usize) -> (i32, i32) {
    match facing {
        0 => (1, 0),
        1 => (0, 1),
        2 => (-1, 0),
        3 => (0, -1),
        _ => {unreachable!()}
    }
}

fn parse_input(s: &str) -> (Vec<Vec<Option<Tile>>>, Vec<Instruction>)
{
    let mut result = Vec::new();
    let last = s.lines().last().unwrap();
    let len = s.lines().collect::<Vec<_>>().len();
    for line in s.lines().take(len - 2) {
        let mut line_vec = Vec::new();
        for ch in line.chars() {
            let r = match ch {
                ' ' => None,
                '.' => Some(Space),
                '#' => Some(Block),
                _ => {unreachable!()}
            };
            line_vec.push(r);
        }
        result.push(line_vec);
    }
    let max = result.iter().map(|line| line.len()).max().unwrap();
    for i in 0..result.len() {
        for _ in result[i].len()..max {
            result[i].push(None);
        }
    }
    let mut instructions = Vec::new();
    for instruction in last.replace("R", " R ").replace("L", " L ").split(" ") {
        let i = match instruction {
            "R" => Right,
            "L" => Left,
            _ => {Forward(instruction.parse().unwrap())}
        };
        instructions.push(i);
    }
    return (result, instructions);
}

fn modulo(a: i32, b: i32) -> i32 {
    ((a % b) + b) % b
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile { Space, Block }
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Instruction { Forward(u32), Right, Left }
