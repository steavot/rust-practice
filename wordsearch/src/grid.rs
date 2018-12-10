// module to hold grid for word search
use crate::direction::Direction;
use crate::gridpoints::Point;

use std::iter::FromIterator;
use std::ops::Neg;

#[derive(Clone)]
pub struct Grid {
    grid: Box<Vec<Vec<char>>>,
}

impl Grid {
    pub fn new(wordgrid: Vec<Vec<char>>) -> Grid {
        Grid {
            grid: Box::new(wordgrid),
        }
    }

    fn char_at_position(&self, letter: char, position: &Point) -> Option<()> {
        match self.grid.get(position.y as usize) {
            // y coordinate off the grid so... nope
            None => None,
            Some(row) => match row.get(position.x as usize) {
                // x coordinate off the grid so... nope
                None => None,
                Some(gridchar) => {
                    if *gridchar == letter {
                        Some(())
                    } else {
                        None
                    }
                }
            },
        }
    }

    // Given a point and a direction of travel, is the word there?
    fn word_in_direction_from_point(
        &self,
        word: &str,
        direction: &Direction,
        start_point: &Point,
    ) -> Option<Point> {
        // Fold over the word, accumulator being the success of
        // the previous character being at the previous point.
        word.chars().fold(
            Some(start_point.translate(&direction.clone().neg(), 1)),
            |acc, x| match acc {
                None => None,
                Some(last_point) => {
                    let point = last_point.translate(direction, 1);
                    match self.char_at_position(x, &point) {
                        None => None,
                        Some(_) => Some(point),
                    }
                }
            },
        )
    }

    // Does the word start at the point we're given?
    fn word_exists_from_point(&self, word: &str, point: &Point) -> Option<Point> {
        // fold over all possible directions, accumulator being success of
        // finding word in previous direction.
        Direction::iter_variants().fold(None, |acc, x| match acc {
            // We've found it already, do nothing.
            Some(end_point) => Some(end_point),
            // We've not found it yet, look in this direction.
            None => self.word_in_direction_from_point(word, &x, point),
        })
    }

    pub fn contains_word(&self, word: &str) -> Option<(Point, Point)> {
        (0..self.grid.len()).fold(
            None,
            // acc: Option<(Point, Point)>
            |acc, y| match acc {
                Some(start_end_points) => Some(start_end_points),
                None => (0..self.grid[0].len()).fold(
                    None,
                    // acc: Option<(Point, Point)>
                    |acc, x| match acc {
                        Some(start_end_points) => Some(start_end_points),
                        None => match self.word_exists_from_point(
                            word,
                            &Point {
                                x: x as isize,
                                y: y as isize,
                            },
                        ) {
                            Some(end_point) => Some((
                                Point {
                                    x: x as isize,
                                    y: y as isize,
                                },
                                end_point,
                            )),
                            None => None,
                        },
                    },
                ),
            },
        )
    }

    pub fn pretty_print(&self) {
        let max_x = self.grid.iter().fold(
            0,
            |acc, x| {
                if acc > x.len() {
                    acc
                } else {
                    x.len()
                }
            }
        );

        let mut boundry_vec = vec!['_'; max_x*2 + 1];
        boundry_vec.insert(0, ' ');
        boundry_vec.insert(0, ' ');
        boundry_vec.push(' ');
        let boundry = String::from_iter(boundry_vec);

        let mut boundry2_vec = vec![' '; max_x*2 + 1];
        boundry2_vec.insert(0, '|');
        boundry2_vec.push('|');
        let boundry2 = String::from_iter(boundry2_vec);

        println!("{:?}", boundry);
        println!("{:?}", boundry2);

        for row in self.grid.iter() {
            let mut row_vec = Vec::new();
            row_vec.push('|');
            row_vec.push(' ');
            for letter in row.iter() {
                row_vec.push(*letter);
                row_vec.push(' ');
            }
            row_vec.push('|');
            let row_string = String::from_iter(row_vec);
            println!("{:?}", row_string);
        }
        let mut boundry3_vec = vec!['_'; max_x*2 + 1];
        boundry3_vec.insert(0, '|');
        boundry3_vec.push('|');
        let boundry3 = String::from_iter(boundry3_vec);
        println!("{:?}", boundry3);
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn char_lookup() {
        let wordgrid = vec![
            vec!['T', 'I', 'T'],
            vec!['A', 'I', 'A'],
            vec!['A', 'I', 'B'],
        ];
        let mygrid = Grid::new(wordgrid);
        let goodpoint = Point { x: 1, y: 2 };

        assert_eq!(mygrid.char_at_position('I', &goodpoint), Some(()));
    }

    #[test]
    fn bad_char_lookup() {
        let wordgrid = vec![
            vec!['T', 'I', 'T'],
            vec!['A', 'I', 'A'],
            vec!['A', 'I', 'B'],
        ];
        let mygrid = Grid::new(wordgrid);
        let goodpoint = Point { x: 1, y: 2 };
        let badpoint = Point { x: -1, y: 2 };

        assert_eq!(mygrid.char_at_position('I', &badpoint), None);
        assert_eq!(mygrid.char_at_position('Z', &goodpoint), None);
    }

    #[test]
    fn word_from_point_given_direction() {
        let wordgrid = vec![
            vec!['T', 'I', 'T'],
            vec!['A', 'I', 'A'],
            vec!['A', 'I', 'B'],
        ];
        let mygrid = Grid::new(wordgrid);
        let myword = "BIT";
        let mydirection = Direction::UpLeft;
        let mypoint = Point { x: 2, y: 2 };
        assert_eq!(
            mygrid.word_in_direction_from_point(&myword, &mydirection, &mypoint),
            Some(Point { x: 0, y: 0 })
        );
    }

    #[test]
    fn word_not_from_point_given_direction() {
        let wordgrid = vec![
            vec!['T', 'I', 'T'],
            vec!['A', 'I', 'A'],
            vec!['A', 'I', 'B'],
        ];
        let mygrid = Grid::new(wordgrid);
        let myword = "BIT";
        let baddirection = Direction::Up;
        let mypoint = Point { x: 2, y: 2 };
        assert_eq!(
            mygrid.word_in_direction_from_point(&myword, &baddirection, &mypoint),
            None
        );

        let baddirection = Direction::Down;
        assert_eq!(
            mygrid.word_in_direction_from_point(&myword, &baddirection, &mypoint),
            None
        );

        let mypoint = Point { x: 0, y: 2 };
        assert_eq!(
            mygrid.word_in_direction_from_point(&myword, &baddirection, &mypoint),
            None
        );
    }

    #[test]
    fn word_found_from_point() {
        let wordgrid = vec![
            vec!['T', 'I', 'T'],
            vec!['A', 'I', 'A'],
            vec!['A', 'I', 'B'],
        ];
        let mygrid = Grid::new(wordgrid);
        let myword = "TIT";
        let mypoint = Point { x: 2, y: 0 };
        assert_eq!(
            mygrid.word_exists_from_point(&myword, &mypoint),
            Some(Point { x: 0, y: 0 })
        );
    }

    #[test]
    fn word_not_found_from_point() {
        let wordgrid = vec![
            vec!['T', 'I', 'T'],
            vec!['A', 'I', 'A'],
            vec!['A', 'I', 'B'],
        ];
        let mygrid = Grid::new(wordgrid);
        let myword = "TIIIT";
        let mypoint = Point { x: 2, y: 0 };
        assert_eq!(mygrid.word_exists_from_point(&myword, &mypoint), None);

        let myword = "TIB";
        assert_eq!(mygrid.word_exists_from_point(&myword, &mypoint), None);

        let myword = "TIB";
        let mypoint = Point { x: 1, y: 1 };
        assert_eq!(mygrid.word_exists_from_point(&myword, &mypoint), None);
    }

    #[test]
    fn word_in_grid() {
        let wordgrid = vec![
            vec!['T', 'I', 'T'],
            vec!['A', 'I', 'A'],
            vec!['A', 'I', 'B'],
        ];
        let mygrid = Grid::new(wordgrid);
        let myword = "TAB";
        assert_eq!(
            mygrid.contains_word(&myword),
            Some((Point { x: 2, y: 0 }, Point { x: 2, y: 2 }))
        );
        let myword = "AB";
        assert_eq!(
            mygrid.contains_word(&myword),
            Some((Point { x: 2, y: 1 }, Point { x: 2, y: 2 }))
        );
        let myword = "BII";
        assert_eq!(mygrid.contains_word(&myword), None);
    }

}
