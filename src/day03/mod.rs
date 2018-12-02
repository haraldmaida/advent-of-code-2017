//! # Day 3: Spiral Memory
//!
//! ## Part 1
//!
//! You come across an experimental new kind of memory stored on an infinite
//! two-dimensional grid.
//!
//! Each square on the grid is allocated in a spiral pattern starting at a
//! location marked 1 and then counting up while spiraling outward. For example,
//! the first few squares are allocated like this:
//!
//! ```text
//! 17  16  15  14  13
//! 18   5   4   3  12
//! 19   6   1   2  11
//! 20   7   8   9  10
//! 21  22  23---> ...
//! ```
//!
//! While this is very space-efficient (no squares are skipped), requested data
//! must be carried back to square 1 (the location of the only access port for
//! this memory system) by programs that can only move up, down, left, or right.
//! They always take the shortest path: the Manhattan Distance between the
//! location of the data and square 1.
//!
//! For example:
//!
//! * Data from square 1 is carried 0 steps, since it's at the access port.
//! * Data from square 12 is carried 3 steps, such as: down, left, left.
//! * Data from square 23 is carried only 2 steps: up twice.
//! * Data from square 1024 must be carried 31 steps.
//!
//! How many steps are required to carry the data from the square identified in
//! your puzzle input all the way to the access port?
//!
//! [Advent of Code 2017 - Day 3](https://adventofcode.com/2017/day/3)

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Loc(pub i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Cycle(pub i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Sector {
    East,
    North,
    West,
    South,
}

impl AsRef<Loc> for Loc {
    fn as_ref(&self) -> &Loc {
        self
    }
}

#[aoc_generator(day3)]
pub fn parse(input: &str) -> Loc {
    Loc(input
        .trim()
        .parse()
        .expect("input must be a positive integer"))
}

#[aoc(day3, part1)]
pub fn port_distance(input: &Loc) -> i32 {
    let pos = position(*input);
    pos.x + pos.y
}

fn position(loc: Loc) -> Coord {
    let c = cycle(loc);
    let s = sector(loc);
    let sn = match s {
        Sector::East => 0,
        Sector::North => 1,
        Sector::West => 2,
        Sector::South => 3,
    };
    let offset = loc.0 - first_loc(c).0 - sn * length(c) / 4;
    let c = c.0;
    match s {
        Sector::East => Coord {
            x: -c + offset + 1,
            y: c,
        },
        Sector::North => Coord {
            x: -c,
            y: -c + offset + 1,
        },
        Sector::West => Coord {
            x: c - offset - 1,
            y: -c,
        },
        Sector::South => Coord {
            x: c,
            y: c - offset - 1,
        },
    }
}

fn cycle(loc: Loc) -> Cycle {
    if loc.0 <= 1 {
        Cycle(0)
    } else {
        Cycle(((((loc.0 - 1) as f64).sqrt() + 1.) / 2.).floor() as i32)
    }
}

fn first_loc(cycle: Cycle) -> Loc {
    let x = 2 * cycle.0 - 1;
    Loc(x * x + 1)
}

fn length(cycle: Cycle) -> i32 {
    8 * cycle.0
}

fn sector(loc: Loc) -> Sector {
    if loc.0 <= 1 {
        return Sector::East;
    }
    let c = cycle(loc);
    let offset = loc.0 - first_loc(c).0;
    let n = length(c);
    match 4 * offset / n {
        0 => Sector::East,
        1 => Sector::North,
        2 => Sector::West,
        3 => Sector::South,
        _ => panic!("undefined"),
    }
}

#[cfg(test)]
mod tests;
