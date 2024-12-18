use std::{fmt::Display, ops::Mul};

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

impl Mul<usize> for Direction {
    type Output = Self;
    fn mul(self, rhs: usize) -> Self::Output {
        Self {
            i: self.i * rhs as isize,
            j: self.j * rhs as isize,
        }
    }
}

impl Direction {
    pub const N: Self = Direction { i: -1, j: 0 };
    pub const E: Self = Direction { i: 0, j: 1 };
    pub const S: Self = Direction { i: 1, j: 0 };
    pub const W: Self = Direction { i: 0, j: -1 };

    pub const NE: Self = Direction { i: -1, j: 1 };
    pub const NW: Self = Direction { i: -1, j: -1 };
    pub const SE: Self = Direction { i: 1, j: 1 };
    pub const SW: Self = Direction { i: 1, j: -1 };

    pub const CROSS: [Self; 4] = [Self::N, Self::E, Self::S, Self::W];
    #[rustfmt::skip]
    pub const OMNI: [Self; 8] = [
        Self::N, Self::E, Self::S, Self::W,
        Self::NE, Self::NW, Self::SE, Self::SW
    ];

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
