#[macro_use] extern crate custom_derive;
#[macro_use] extern crate enum_derive;
extern crate failure;

mod direction;
mod gridpoints;

use std::ops::Neg;
use direction::Direction;
use gridpoints::Point;
use failure::Error;

const N: usize = 3;
static GRID: [[char; N]; N] = [['T', 'I', 'T'], ['A', 'I', 'A'], ['A', 'A', 'B']];
type Grid = [[char; N]; N];

fn find_word_in_grid(gird: &Grid, word: &str) -> Result<(Point, Point), ()> {

}

fn find_word_from_point(grid: &Grid, point: &Point, word: &str) -> Result<(Point, Point), ()> {
    match Direction::iter_variants().fold(Err(()), |acc, x| match (acc, x) {
        (Ok(end_point), _) => Ok(end_point),
        (Err(_), direction) => find_word_in_direction(point, grid, word, direction),
    }) {
        Ok(end_point) => Ok((point.clone(), end_point)),
        Err(_) => Err(()),
    }
}

fn find_word_in_direction(
    start: &Point,
    grid: &Grid,
    word: &str,
    direction: Direction,
) -> Result<Point, ()> {
    word_fits_in_direction(start, word, direction.clone())?;
    let mut position = Point {
        x: start.x,
        y: start.y,
    };

    for letter in word.chars() {
        position = position.new_translated(direction.clone(), 1);
        char_at_position(grid, letter, &position)?;
    }

    let opposite_direction = direction.neg();
    Ok(position.new_translated(opposite_direction, 1))
}

fn word_fits_in_direction(start: &Point, word: &str, direction: Direction) -> Result<(), ()> {
    position_on_grid(start)?;
    let len = word.len();
    let position = start.new_translated(direction, len);
    position_on_grid(&position)
}

fn char_at_position(grid: &Grid, letter:char, position: &Point) -> Result<(),()> {
    position_on_grid(position)?;
    if grid[position.x as usize][position.y as usize] == letter {
        Ok(())
    } else {
        Err(())
    }
}

fn position_on_grid(position: &Point) -> Result<(), ()> {
    if position.x < 0 || position.y < 0 || position.x as usize >= N || position.y as usize >= N {
        Err(())
    } else {
        Ok(())
    }
}

fn main() {
    println!("Hello, world!");
}
