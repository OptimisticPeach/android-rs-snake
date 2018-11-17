#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum Direction {
    Up,
    Right,
    Left,
    Down,
    Middle,
}

impl Direction {
    pub fn add_to(&self, (x, y): (usize, usize)) -> (isize, isize) {
        let (x, y) = (x as isize, y as isize);
        let (x, y) = match *self {
            Direction::Up => (x, y - 1),
            Direction::Right => (x + 1, y),
            Direction::Left => (x - 1, y),
            Direction::Down => (x, y + 1),
            Direction::Middle => (x, y),
        };
        (x, y)
    }

    pub fn add_to_isize(&self, (x, y): (isize, isize)) -> (isize, isize) {
        let (x, y) = match *self {
            Direction::Up => (x, y - 1),
            Direction::Right => (x + 1, y),
            Direction::Left => (x - 1, y),
            Direction::Down => (x, y + 1),
            Direction::Middle => (x, y),
        };
        (x, y)
    }
}