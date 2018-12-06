#[macro_use] extern crate custom_derive;
#[macro_use] extern crate enum_derive;
extern crate failure;

mod direction;
mod gridpoints;

use direction::Direction;
use gridpoints::Point;
use failure::Error;

// const N: usize = 3;
// static GRID: [[char; N]; N] = [['T', 'I', 'T'], ['A', 'I', 'A'], ['A', 'A', 'B']];
// type Grid = [[char; N]; N];
//
// fn find_word_in_direction(
//     start: &Point,
//     grid: &Grid,
//     word: &str,
//     direction: &Direction,
// ) -> Result<Point, ()> {
//     word_fits_in_direction(start, word, direction)?;
//     let mut position = Point {
//         x: start.x,
//         y: start.y,
//     };
//
//     for letter in word.chars() {
//         position = position.new_translated(direction, 1);
//         char_at_position(grid, letter, &position)?;
//     }
//
//     let delta: isize = -1;
//     Ok(position.new_translated(&direction, delta))
// }
//
// fn word_fits_in_direction(start: &Point, word: &str, direction: &Direction) -> Result<(), ()> {
//     let len = word.len();
//     let mut position = Point {
//         x: start.x,
//         y: start.y,
//     };
//     position = position.new_translated(&direction, len.into());
//     if position.x < 0 || position.y < 0 || position.x >= N || position.y >= N {
//         Err(())
//     } else {
//         Ok(())
//     }
// }
//
// fn char_at_position(grid: &Grid, letter:char, position: &Point) -> Result<(),()> {
//     if grid[position.x][position.y] == letter {
//         Ok(())
//     } else {
//         Err(())
//     }
// }

fn main() {
    println!("Hello, world!");
}
