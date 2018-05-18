use std::fmt;

#[derive(Clone, PartialEq, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn turn_right(&self) -> Direction {
        match self {
            &Direction::Up => Direction::Right,
            &Direction::Down => Direction::Left,
            &Direction::Left => Direction::Up,
            &Direction::Right => Direction::Down,
        }
    }

    pub fn turn_left(&self) -> Direction {
        match self {
            &Direction::Up => Direction::Left,
            &Direction::Down => Direction::Right,
            &Direction::Left => Direction::Down,
            &Direction::Right => Direction::Up,
        }
    }

    pub fn reverse(&self) -> Direction {
        match self {
            &Direction::Up => Direction::Down,
            &Direction::Down => Direction::Up,
            &Direction::Left => Direction::Right,
            &Direction::Right => Direction::Left,
        }
    }

    pub fn step_offset(&self) -> (i32, i32) {
        match self {
            &Direction::Up => (0, 1),
            &Direction::Down => (0, -1),
            &Direction::Left => (-1, 0),
            &Direction::Right => (1, 0),
        }
    }
}


impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Direction::Up => write!(f, "U"),
            &Direction::Down => write!(f, "D"),
            &Direction::Left => write!(f, "L"),
            &Direction::Right => write!(f, "R"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn turn_right() {
        assert_eq!(Direction::Up.turn_right(), Direction::Right);
        assert_eq!(Direction::Down.turn_right(), Direction::Left);
        assert_eq!(Direction::Left.turn_right(), Direction::Up);
        assert_eq!(Direction::Right.turn_right(), Direction::Down);
    }

    #[test]
    fn turn_left() {
        assert_eq!(Direction::Up.turn_left(), Direction::Left);
        assert_eq!(Direction::Down.turn_left(), Direction::Right);
        assert_eq!(Direction::Left.turn_left(), Direction::Down);
        assert_eq!(Direction::Right.turn_left(), Direction::Up);
    }

    #[test]
    fn reverse() {
        assert_eq!(Direction::Up.reverse(), Direction::Down);
        assert_eq!(Direction::Down.reverse(), Direction::Up);
        assert_eq!(Direction::Left.reverse(), Direction::Right);
        assert_eq!(Direction::Right.reverse(), Direction::Left);
    }
}
