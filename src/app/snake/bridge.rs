pub struct Bridge {
    pub pos: (usize, usize),
}

impl Bridge {
    pub fn new(x: usize, y: usize) -> Self {
        Self { pos: (x, y) }
    }
}

impl PartialEq<(usize, usize)> for Bridge {
    fn eq(&self, other: &(usize, usize)) -> bool {
        self.pos.0 == other.0 && self.pos.1 == other.1
    }
}