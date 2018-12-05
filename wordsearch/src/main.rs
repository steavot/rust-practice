extern crate failure;

use failure::Error;
use std::ops::Neg;

const N: usize = 3;
static GRID: [[char; N]; N] = [['T', 'I', 'T'], ['A', 'I', 'A'], ['A', 'A', 'B']];
type Grid = [[char; N]; N];

#[derive(Debug, PartialEq)]
enum Direction {
    UpLeft,
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
}

// impl Neg for Direction {
//     type Output = Direction;
//
//     fn neg(self) -> Direction {
//         match self {
//             Direction::Up => Direction::Down,
//             Direction::Down => Direction::Up,
//             Direction::Left => Direction::Right,
//             Direction::Right => Direction::Left,
//             Direction::UpRight => Direction::DownLeft,
//             Direction::DownRight => Direction::UpLeft,
//             Direction::UpLeft => Direction::DownRight,
//             Direction::DownLeft => Direction::UpRight,
//         }
//     }
// }

struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new_translated(&mut self, direction: &Direction, delta: isize) -> Point {
        match direction {
            Direction::Up => Point {
                x: self.x,
                y: self.y - delta,
            },
            Direction::Down => Point {
                x: self.x,
                y: self.y + delta,
            },
            Direction::Left => Point {
                x: self.x - delta,
                y: self.y,
            },
            Direction::Right => Point {
                x: self.x + delta,
                y: self.y,
            },
            Direction::UpRight => Point {
                x: self.x + delta,
                y: self.y - delta,
            },
            Direction::UpLeft => Point {
                x: self.x + delta,
                y: self.y - delta,
            },
            Direction::DownRight => Point {
                x: self.x + delta,
                y: self.y - delta,
            },
            Direction::DownLeft => Point {
                x: self.x + delta,
                y: self.y - delta,
            },
        }
    }
}

// #[derive(Debug)]
// struct DoesntFitError {
//     details: String
// }
//
// impl DoesntFitError {
//     fn new(msg: &str) -> DoesntFitError {
//         DoesntFitError{details: msg.to_string()}
//     }
// }

fn find_word_in_direction(
    start: &Point,
    grid: &Grid,
    word: &str,
    direction: &Direction,
) -> Result<Point, ()> {
    word_fits_in_direction(start, word, direction)?;
    let mut position = Point {
        x: start.x,
        y: start.y,
    };

    for letter in word.chars() {
        position = position.new_translated(direction, 1);
        char_at_position(grid, letter, &position)?;
    }

    let delta: isize = -1;
    Ok(position.new_translated(&direction, delta))
}

fn word_fits_in_direction(start: &Point, word: &str, direction: &Direction) -> Result<(), ()> {
    let len = word.len();
    let mut position = Point {
        x: start.x,
        y: start.y,
    };
    position = position.new_translated(&direction, len.into());
    if position.x < 0 || position.y < 0 || position.x >= N || position.y >= N {
        Err(())
    } else {
        Ok(())
    }
}

fn char_at_position(grid: &Grid, letter:char, position: &Point) -> Result<(),()> {
    if grid[position.x][position.y] == letter {
        Ok(())
    } else {
        Err(())
    }
}

fn main() {
    println!("Hello, world!");
}
