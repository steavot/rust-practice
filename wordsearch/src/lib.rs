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
use std::sync::{Arc, Mutex};
use std::thread;

pub fn find_words_in_series(wordgrid: Vec<Vec<char>>, words: Vec<String>) -> HashMap<String, Option<(Point, Point)>>{
    let grid = Grid::new(wordgrid);
    // taking advantage of non-lexical lifetimes in 2018
    words.iter().fold(HashMap::new(), |mut acc, word| {
        acc.insert(word.clone(), grid.contains_word(&word));
        acc
    })
}

pub fn find_words_in_channels(wordgrid: Vec<Vec<char>>, words: Vec<String>) -> HashMap<String, Option<(Point, Point)>>{
    let mastergrid = Grid::new(wordgrid);
    let (tx, rx) = mpsc::channel();

    for word in words {
        let grid = mastergrid.clone();
        let transmit = mpsc::Sender::clone(&tx);
        thread::spawn(move || {
            transmit.send((word.clone(), grid.contains_word(&word))).unwrap();
        });
    }
    drop(tx);

    let mut answers = HashMap::new();
    for (word, answer) in rx {
        answers.insert(word, answer);
    }
    answers
}

pub fn find_words_with_shared_state(wordgrid: Vec<Vec<char>>, words: Vec<String>) -> HashMap<String, Option<(Point, Point)>> {
    let answers = Arc::new(Mutex::new(HashMap::new()));
    let grid = Arc::new(Grid::new(wordgrid));
    let mut handles = vec![];

    for word in words {
        let answers = Arc::clone(&answers);
        let grid = Arc::clone(&grid);
        let handle = thread::spawn(move || {
            let mut ans = answers.lock().unwrap();
            ans.insert(word.clone(), grid.contains_word(&word));
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let answers = answers.lock().unwrap();
    answers.clone()
}

fn pretty_print_grid(wordgrid: Vec<Vec<char>>) {
    let grid = Grid::new(wordgrid);
    grid.pretty_print();
}

fn pretty_print_answers(answers: HashMap<String, Option<(Point, Point)>>) {
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
            "CAROL".to_string(),
            "MERRY".to_string(),
            "STAR".to_string(),
            "CHRISTMAS".to_string(),
            "PRESENTS".to_string(),
            "TREE".to_string(),
            "HOLLY".to_string(),
            "SANTA".to_string(),
            "WREATH".to_string(),
        ];

        pretty_print_grid(wordgrid.clone());
        let answers = find_words_in_series(wordgrid, words);
        pretty_print_answers(answers);
    }

    // cargo test test_parallel_state -- --nocapture
    #[test]
    fn test_parallel_state() {
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
            "CAROL".to_string(),
            "MERRY".to_string(),
            "STAR".to_string(),
            "CHRISTMAS".to_string(),
            "PRESENTS".to_string(),
            "TREE".to_string(),
            "HOLLY".to_string(),
            "SANTA".to_string(),
            "WREATH".to_string(),
        ];

        pretty_print_grid(wordgrid.clone());
        let answers = find_words_with_shared_state(wordgrid, words);
        pretty_print_answers(answers);
    }

    // cargo test test_parallel_channels -- --nocapture
    #[test]
    fn test_parallel_channels() {
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
            "CAROL".to_string(),
            "MERRY".to_string(),
            "STAR".to_string(),
            "CHRISTMAS".to_string(),
            "PRESENTS".to_string(),
            "TREE".to_string(),
            "HOLLY".to_string(),
            "SANTA".to_string(),
            "WREATH".to_string(),
        ];

        pretty_print_grid(wordgrid.clone());
        let answers = find_words_in_channels(wordgrid, words);
        pretty_print_answers(answers);
    }
}
