// module for a point in the same co-ordinate system that we put our grid.
//
// Co-ordinate system axes orientated like this:
//
//          -3
//          -2
//          -1
// -3 -2 -1  0  1  2  3
//           1
//           2
//           3
//
use crate::direction::Direction;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point ({}, {})", self.x, self.y)
    }
}

impl Point {
    pub fn translate(&self, direction: &Direction, udelta: usize) -> Point {
        let delta = udelta as isize;
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
                x: self.x - delta,
                y: self.y - delta,
            },
            Direction::DownRight => Point {
                x: self.x + delta,
                y: self.y + delta,
            },
            Direction::DownLeft => Point {
                x: self.x - delta,
                y: self.y + delta,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clone_and_display() {
        let a = Point { x: 1, y: 2 };
        let _b = a.clone();
        println!("{}", a);
    }

    #[test]
    fn translation() {
        let a = Point { x: 1, y: 2 };
        let direction = Direction::DownLeft;
        let b = a.translate(&direction, 2);
        println!("{}", a);
        assert_eq!(b, Point { x: -1, y: 4 });
    }
}
