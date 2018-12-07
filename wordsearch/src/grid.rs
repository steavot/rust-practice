// module to hold grid for word search
use direction::Direction;
use gridpoints::Point;

use std::ops::Neg;

pub struct Grid {
    grid: Box<Vec<Vec<char>>>,
}

impl Grid {
    pub fn new(wordgrid: Vec<Vec<char>>) -> Grid {
        Grid {
            grid: Box::new(wordgrid),
        }
    }

    fn char_at_position(&self, letter: char, position: &Point) -> Result<(), ()> {
        match self.grid.get(position.y as usize) {
            // y coordinate off the grid so... nope
            None => Err(()),
            Some(row) => match row.get(position.x as usize) {
                // x coordinate off the grid so... nope
                None => Err(()),
                Some(gridchar) => if *gridchar == letter {
                    Ok(())
                } else {
                    Err(())
                },
            },
        }
    }

    // Given a point and a direction of travel, is the word there?
    fn word_in_direction_from_point(
        &self,
        word: &str,
        direction: &Direction,
        start_point: &Point,
    ) -> Result<Point, ()> {
        // Fold over the word, accumulator being the success of
        // the previous charater being at the previous point.
        word.chars().fold(
            Ok(start_point.translate(&direction.clone().neg(), 1)),
            |acc, x| match acc {
                Err(_) => Err(()),
                Ok(last_point) => {
                    let point = last_point.translate(direction, 1);
                    match self.char_at_position(x, &point) {
                        Err(_) => Err(()),
                        Ok(_) => Ok(point),
                    }
                }
            },
        )
    }

    // Does the word start at the point we're given?
    fn word_exists_from_point(&self, word: &str, point: &Point) -> Result<Point, ()> {
        // fold over all possible directions, accumulator being success
        // finding word in previous direction.
        Direction::iter_variants().fold(
            Err(()),
            |acc, x| match (acc, x) {
                // OK we've found it already, do nothing.
                (Ok(end_point), _) => Ok(end_point),
                // Err we've not found it yet, look in this direction.
                (Err(_), direction) => self.word_in_direction_from_point(word, &direction, point),
            },
        )
    }

    pub fn contains_word(&self, word: &str) -> Result<(Point, Point), ()> {
        (0..self.grid.len()).fold(
            Err(()),
            // acc: Result<Point, Point>
            |acc, y| match (acc, y) {
                (Ok((start_point, end_point)), _) => Ok((start_point, end_point)),
                (Err(_), y) => (0..self.grid[0].len()).fold(
                    Err(()),
                    // acc: Result<Point,Point>
                    |acc, x| match (acc, x) {
                        (Ok((start_point, end_point)), _) => Ok((start_point, end_point)),
                        (Err(_), x) => match self.word_exists_from_point(word, &Point{x: x as isize, y:y as isize}) {
                            Ok(end_point) => Ok((Point{x: x as isize, y: y as isize}, end_point)),
                            Err(_) => Err(())
                        }
                    }
                )
            }
        )
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn char_lookup() -> Result<(), ()> {
        let wordgrid = vec![
            vec!['T', 'I', 'T'],
            vec!['A', 'I', 'A'],
            vec!['A', 'I', 'B'],
        ];
        let mygrid = Grid::new(wordgrid);
        let goodpoint = Point { x: 1, y: 2 };

        mygrid.char_at_position('I', &goodpoint)
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

        assert!(mygrid.char_at_position('I', &badpoint).is_err());
        assert!(mygrid.char_at_position('Z', &goodpoint).is_err());
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
        assert_eq!(mygrid.word_in_direction_from_point(&myword, &mydirection, &mypoint), Ok(Point { x:0, y:0 }));
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
        assert_eq!(mygrid.word_in_direction_from_point(&myword, &baddirection, &mypoint), Err(()));

        let baddirection = Direction::Down;
        assert_eq!(mygrid.word_in_direction_from_point(&myword, &baddirection, &mypoint), Err(()));

        let mypoint = Point { x: 0, y: 2 };
        assert_eq!(mygrid.word_in_direction_from_point(&myword, &baddirection, &mypoint), Err(()));
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
        assert_eq!(mygrid.word_exists_from_point(&myword, &mypoint), Ok(Point { x:0, y:0 }));
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
        assert_eq!(mygrid.word_exists_from_point(&myword, &mypoint), Err(()));

        let myword = "TIB";
        assert_eq!(mygrid.word_exists_from_point(&myword, &mypoint), Err(()));

        let myword = "TIB";
        let mypoint = Point { x: 1, y: 1 };
        assert_eq!(mygrid.word_exists_from_point(&myword, &mypoint), Err(()));
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
        assert_eq!(mygrid.contains_word(&myword), Ok((Point { x:2, y:0 }, Point {x:2, y:2})));
        let myword = "AB";
        assert_eq!(mygrid.contains_word(&myword), Ok((Point { x:2, y:1 }, Point {x:2, y:2})));
        let myword = "BII";
        assert_eq!(mygrid.contains_word(&myword), Err(()));
    }






}
