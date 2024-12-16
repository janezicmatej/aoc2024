use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Direction {
    pub i: isize,
    pub j: isize,
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.i, self.j)
    }
}

impl Direction {
    pub const NORTH: Self = Direction { i: -1, j: 0 };
    pub const EAST: Self = Direction { i: 0, j: 1 };
    pub const SOUTH: Self = Direction { i: 1, j: 0 };
    pub const WEST: Self = Direction { i: 0, j: -1 };

    pub fn new(i: isize, j: isize) -> Self {
        Self { i, j }
    }

    pub fn rotate_left(self) -> Self {
        Self {
            i: -self.j,
            j: self.i,
        }
    }

    pub fn rotate_right(self) -> Self {
        Self {
            i: self.j,
            j: -self.i,
        }
    }
}

#[macro_export]
macro_rules! dir {
    ($i:literal, $j:literal) => {
        Direction { i: $i, j: $j }
    };
}
