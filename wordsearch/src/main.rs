#[macro_use]
extern crate custom_derive;
#[macro_use]
extern crate enum_derive;
extern crate failure;

mod direction;
mod grid;
mod gridpoints;

use direction::Direction;
use failure::Error;
use gridpoints::Point;
use std::ops::Neg;

fn main() {
    println!("Hello, world!");
}
