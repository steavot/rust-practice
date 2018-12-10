#[macro_use]
extern crate custom_derive;
#[macro_use]
extern crate enum_derive;


mod direction;
mod grid;
mod gridpoints;

fn find_words_in_series() {


}

#[cfg(test)]
mod tests {
    use super::*;

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

    }
}
