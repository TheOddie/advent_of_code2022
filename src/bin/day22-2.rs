use std::ops::Add;
use std::slice::Iter;

fn main() {
    let cube = parse_input(include_str!("../input/day22.txt"))
        .find_exits()
        .solve();
    println!("Ans: {:?}", cube.cursor.unwrap().password(cube.size));
}

fn parse_input(s: &str) -> Cube {
    let height = s.lines()
        .position(|l| l.is_empty())
        .unwrap();
    let width = s.lines()
        .take(height)
        .map(|l| l.len())
        .max()
        .unwrap();
    let size = gcd::binary_usize(width, height);

    let instructions = s.lines()
        .nth(height + 1)
        .unwrap()
        .replace("R", " R ")
        .replace("L", " L ")
        .split_whitespace()
        .map(|x| {
            match x {
                "R" => Instruction::Right,
                "L" => Instruction::Left,
                &_ => Instruction::Forward(x.parse().unwrap())
            }
        })
        .collect();

    let mut result = Cube {
        faces: Vec::new(),
        size,
        cursor: None,
        instructions
    };

    for (y, line) in s.lines().take(height).enumerate() {
        for (x, ch) in line.chars().enumerate() {
            match ch {
                '.' => result.put(Tile::Space, Pos(x, y)),
                '#' => result.put(Tile::Block, Pos(x, y)),
                _ => {}
            }
        }
    }

    result
}

#[derive(Debug)]
struct Cube {
    faces: Vec<Face>,
    size: usize,
    cursor: Option<Cursor>,
    instructions: Vec<Instruction>
}

impl Cube {

    fn put(&mut self, tile: Tile, pos: Pos) {
        let face_pos = Pos(pos.0 / self.size, pos.1 / self.size);
        let inner_pos = Pos(pos.0 % self.size, pos.1 % self.size);

        if let Some(c) = self.faces.iter_mut().find(|x| x.pos == face_pos) {
            c.content[inner_pos.1 * self.size + inner_pos.0] = tile;
        }
        else {
            let mut c = Face {
                content: vec![Tile::Space; self.size * self.size],
                pos: face_pos,
                exits: vec![None, None, None, None],
            };
            c.content[inner_pos.1 * self.size + inner_pos.0] = tile;
            self.faces.push(c);
        }

        if self.cursor.is_none() && tile == Tile::Space {
            self.cursor = Some(Cursor { dir: Dir::Right, face: face_pos, inner: inner_pos });
        }
    }

    fn get(&self, face: Pos, inner: Pos) -> Tile {
        self.faces.iter()
            .find(|x| x.pos == face)
            .unwrap().content[inner.1 * self.size + inner.0]
    }

    fn find_exits(mut self) -> Self {
        static POS_MODS: [(i64, i64); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
        let maps = self.faces.clone();
        for map in self.faces.iter_mut() {
            for (&(px, py), &d) in POS_MODS.iter().zip(Dir::iterator()) {
                if let Some(n) = maps.iter().find(|m| m.pos == map.pos.pos_mod(px, py)) {
                    map.exits[d.idx()] = Some(Exit(n.pos, d));
                }
            }
        }
        loop {
            let maps = self.faces.clone();
            for map in maps {
                for d in 0..4 {
                    if let Some(exit1) = map.exits[d] {
                        if let Some(exit2) = map.exits[(d + 1) % 4] {
                            let face1 = self.faces.iter_mut().find(|m| m.pos == exit1.0).unwrap();
                            face1.exits[exit1.1.turn_right().idx()] = Some(Exit(exit2.0, exit2.1.turn_right()));
                            let face2 = self.faces.iter_mut().find(|m| m.pos == exit2.0).unwrap();
                            face2.exits[exit2.1.turn_left().idx()] = Some(Exit(exit1.0, exit1.1.turn_left()));
                        }
                    }
                }
            }
            if self.faces.iter()
                .map(|m| m.exits.iter()
                    .filter(|e| e.is_some()).count())
                .sum::<usize>() == 24
            {
                break;
            }
        }
        self
    }

    fn step_dir(&mut self, dir: Dir) {
        static POS_MODS: [(i64, i64); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
        let cursor = self.cursor.unwrap();
        let map_pos = cursor.face;
        let inner_pos = cursor.inner;
        let (new_dir, new_map, new_inner) = if match dir {
            Dir::Up => inner_pos.1 == 0,
            Dir::Down => inner_pos.1 == self.size-1,
            Dir::Left => inner_pos.0 == 0,
            Dir::Right => inner_pos.0 == self.size-1,
        } {
            let map = self.faces.iter().find(|x| x.pos == map_pos).unwrap();
            let exit = map.exits[dir.idx()].unwrap();
            (exit.1, exit.0, exit.1.enter_map(match dir {
                Dir::Up => inner_pos.0,
                Dir::Down => self.size-inner_pos.0-1,
                Dir::Left => self.size-inner_pos.1-1,
                Dir::Right => inner_pos.1,
            }, self.size))
        } else {
            (cursor.dir, map_pos, inner_pos + POS_MODS[dir.idx()])
        };
        if self.get(new_map, new_inner) == Tile::Space {
            self.cursor = Some(Cursor {
                dir: new_dir,
                face: new_map,
                inner: new_inner
            });
        }
    }

    fn turn_left(&mut self) {
        let cursor = self.cursor.unwrap();
        self.cursor = Some(Cursor { dir: cursor.dir.turn_left(), ..cursor });
    }

    fn turn_right(&mut self) {
        let cursor = self.cursor.unwrap();
        self.cursor = Some(Cursor { dir: cursor.dir.turn_right(), ..cursor });
    }

    fn step(&mut self) {
        self.step_dir(self.cursor.unwrap().dir);
    }

    fn solve(mut self) -> Self {
        let instructions = self.instructions.clone();
        for i in instructions.iter() {
            match i {
                Instruction::Right => self.turn_right(),
                Instruction::Left => self.turn_left(),
                Instruction::Forward(n) => (0..*n).for_each(|_| self.step()),
            }
        }
        self
    }

}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Cursor {
    dir: Dir,
    face: Pos,
    inner: Pos
}

impl Cursor {
    fn password(&self, size: usize) -> usize {
        let x = self.face.0 * size + self.inner.0 + 1;
        let y = self.face.1 * size + self.inner.1 + 1;
        let dir = self.dir.idx();
        return y * 1000 + x * 4 + dir;
    }
}

#[derive(Debug, Clone)]
struct Face {
    content: Vec<Tile>,
    pos: Pos,
    exits: Vec<Option<Exit>>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile { Space, Block }
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Instruction { Forward(u32), Right, Left }
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Pos(usize, usize);
impl Pos {
    fn pos_mod(&self, by_x: i64, by_y: i64) -> Pos {
        let new_x = ((((self.0 as i64 + by_x) % 5) + 5) % 5) as usize;
        let new_y = ((((self.1 as i64 + by_y) % 5) + 5) % 5) as usize;
        Pos(new_x, new_y)
    }
}

impl Add<(i64, i64)> for Pos{
    type Output = Pos;

    fn add(self, rhs: (i64, i64)) -> Self::Output {
        Pos((self.0 as i64 + rhs.0) as usize, (self.1 as i64 + rhs.1) as usize)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Dir {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl Dir {
    fn iterator() -> Iter<'static, Dir> {
        static DIRS: [Dir; 4] = [Dir::Right, Dir::Down, Dir::Left, Dir::Up];
        DIRS.iter()
    }
    fn idx(&self) -> usize {
        match self {
            Dir::Right => 0,    //   3
            Dir::Down => 1,     // 2 . 0
            Dir::Left => 2,     //   1
            Dir::Up => 3
        }
    }

    fn turn_right(&self) -> Dir {
        Dir::from_usize((self.idx() + 1) % 4)
    }

    fn turn_left(&self) -> Dir {
        Dir::from_usize((self.idx() + 3) % 4)
    }

    fn from_usize(d: usize) -> Dir {
        match d {
            0 => Dir::Right,
            1 => Dir::Down,
            2 => Dir::Left,
            3 => Dir::Up,
            _ => panic!("Unknown directon")
        }
    }

    fn enter_map(&self, offset: usize, size: usize) -> Pos {
        match self {
            Dir::Up => Pos(offset, size-1),
            Dir::Down => Pos(size-offset-1, 0),
            Dir::Left => Pos(size-1, size-offset-1),
            Dir::Right => Pos(0, offset),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Exit(Pos, Dir);
