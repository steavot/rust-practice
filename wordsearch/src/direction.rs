// Module to hold and enum that encodes the directions
// one can search for words in 2D grid
use std::ops::Neg;

custom_derive! {
    #[derive(Debug, PartialEq, Eq, EnumDisplay, NextVariant, PrevVariant, Clone,
        IterVariants(DirectionVariants), IterVariantNames(DirectionVariantNames))]
    pub enum Direction {
        Up,
        UpRight,
        Right,
        DownRight,
        Down,
        DownLeft,
        Left,
        UpLeft,
    }
}

impl Neg for Direction {
    // IDK why this line with type Output is needed
    type Output = Direction;
    fn neg(self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::UpRight => Direction::DownLeft,
            Direction::DownRight => Direction::UpLeft,
            Direction::UpLeft => Direction::DownRight,
            Direction::DownLeft => Direction::UpRight,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::Map;

    #[test]
    fn print_direction() {
        let x = Direction::UpLeft;
        println!("{}", x);
    }

    #[test]
    fn negation() {
        let x = Direction::Down;
        let y = Direction::Up;
        assert_eq!(x, y.neg());
    }

    #[test]
    fn all_variants() {
        let vars: DirectionVariants = Direction::iter_variants();
        assert_eq!(vars.collect::<Vec<_>>(), vec![
                   Direction::Up,
                   Direction::UpRight,
                   Direction::Right,
                   Direction::DownRight,
                   Direction::Down,
                   Direction::DownLeft,
                   Direction::Left,
                   Direction::UpLeft,
        ]);
    }

    #[test]
    fn consecutive_variants() {
        assert_eq!(Direction::DownLeft.next_variant(), Some(Direction::Left));
        assert_eq!(Direction::DownLeft.prev_variant(), Some(Direction::Down));
        assert_eq!(Direction::UpLeft.next_variant(), None);
        assert_eq!(Direction::Up.prev_variant(), None);
    }

    #[test]
    fn all_variants_displayed() {
        let varnames: DirectionVariantNames = Direction::iter_variant_names();
        assert_eq!(varnames.collect::<Vec<_>>(), vec![
            "Up", "UpRight", "Right", "DownRight", "Down", "DownLeft", "Left", "UpLeft",
        ]);
    }

    #[test]
    fn all_variants_negated() {
        let negvars = Direction::iter_variants().map(|var| var.neg());
        assert_eq!(negvars.collect::<Vec<_>>(), vec![
            Direction::Down,
            Direction::DownLeft,
            Direction::Left,
            Direction::UpLeft,
            Direction::Up,
            Direction::UpRight,
            Direction::Right,
            Direction::DownRight,
        ]);
    }
}

