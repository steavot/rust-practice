#[macro_use]
extern crate custom_derive;
#[macro_use]
extern crate enum_derive;

mod direction;
mod grid;
mod gridpoints;

use crate::gridpoints::Point;
use crate::grid::Grid;
use std::collections::HashMap;
use std::sync::mpsc;
use std::thread;

pub fn find_words_in_series(wordgrid: Vec<Vec<char>>, words: Vec<&str>) -> HashMap<&str, Option<(Point, Point)>>{
    let grid = Grid::new(wordgrid);
    // taking advantage of non-lexical lifetimes in 2018
    words.iter().fold(HashMap::new(), |mut acc, word| {
        acc.insert(word, grid.contains_word(word));
        acc
    })
}

pub fn find_words_in_channels<'a>(wordgrid: Vec<Vec<char>>, words: &'a Vec<&str>) -> HashMap<&'a str, Option<(Point, Point)>>{
    let mastergrid = Grid::new(wordgrid);
    let (tx, rx) = mpsc::channel();

    for word in words {
        let this_word = word.clone();
        let grid = mastergrid.clone();
        let transmit = mpsc::Sender::clone(&tx);
        thread::spawn(move || {
            transmit.send((word, grid.contains_word(word))).unwrap();
        });
    }

    let mut answers = HashMap::new();
    for (word, answer) in rx {
        answers.insert(*word, answer);
    }
    answers
}

fn pretty_print_grid(wordgrid: Vec<Vec<char>>) {
    let grid = Grid::new(wordgrid);
    grid.pretty_print();
}

fn pretty_print_answers(answers: HashMap<&str, Option<(Point, Point)>>) {
    for (word, answer) in &answers {
        match answer {
            None => println!("{}: Not Found", word),
            Some((start, end)) => println!("{}: {}, {}", word, start, end),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // cargo test test_series -- --nocapture
    #[test]
    fn test_series() {
        let wordgrid = vec![
            vec!['N', 'D', 'F', 'T', 'R', 'E', 'E', 'E', 'A', 'P'],
            vec!['O', 'P', 'R', 'Z', 'F', 'X', 'K', 'U', 'O', 'R'],
            vec!['E', 'Q', 'U', 'R', 'S', 'A', 'N', 'T', 'A', 'E'],
            vec!['C', 'H', 'R', 'I', 'S', 'T', 'M', 'A', 'S', 'S'],
            vec!['M', 'E', 'R', 'R', 'Y', 'V', 'E', 'N', 'S', 'E'],
            vec!['C', 'A', 'R', 'O', 'L', 'U', 'R', 'V', 'I', 'N'],
            vec!['P', 'S', 'B', 'M', 'J', 'M', 'K', 'T', 'Q', 'T'],
            vec!['U', 'T', 'H', 'O', 'L', 'L', 'Y', 'Y', 'V', 'S'],
            vec!['B', 'A', 'H', 'U', 'Y', 'T', 'O', 'G', 'Z', 'M'],
            vec!['A', 'R', 'E', 'W', 'R', 'E', 'A', 'T', 'H', 'T'],
        ];

        let words = vec![
            "CAROL",
            "MERRY",
            "STAR",
            "CHRISTMAS",
            "PRESENTS",
            "TREE",
            "HOLLY",
            "SANTA",
            "WREATH",
        ];

        pretty_print_grid(wordgrid.clone());
        let answers = find_words_in_series(wordgrid, words);
        pretty_print_answers(answers);
    }

}
