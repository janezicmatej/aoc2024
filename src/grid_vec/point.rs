use super::Direction;

use std::{
    fmt::Display,
    ops::{Add, AddAssign},
};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Point {
    pub i: usize,
    pub j: usize,
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.i, self.j)
    }
}

impl Add<Direction> for Point {
    type Output = Self;
    fn add(self, rhs: Direction) -> Self::Output {
        Self {
            i: self.i.wrapping_add(rhs.i as usize),
            j: self.j.wrapping_add(rhs.j as usize),
        }
    }
}

impl AddAssign<Direction> for Point {
    fn add_assign(&mut self, rhs: Direction) {
        *self = *self + rhs
    }
}

impl Point {
    pub fn new(i: usize, j: usize) -> Self {
        Self { i, j }
    }

    pub fn n(self) -> Self {
        Self {
            i: self.i.wrapping_sub(1),
            j: self.j,
        }
    }

    pub fn s(self) -> Self {
        Self {
            i: self.i.wrapping_add(1),
            j: self.j,
        }
    }

    pub fn e(self) -> Self {
        Self {
            i: self.i,
            j: self.j.wrapping_sub(1),
        }
    }

    pub fn w(self) -> Self {
        Self {
            i: self.i,
            j: self.j.wrapping_add(1),
        }
    }

    pub fn ne(self) -> Self {
        self.n().e()
    }

    pub fn nw(self) -> Self {
        self.n().w()
    }

    pub fn se(self) -> Self {
        self.s().e()
    }

    pub fn sw(self) -> Self {
        self.s().w()
    }
}

#[macro_export]
macro_rules! pnt {
    ($i:literal, $j:literal) => {
        Point { i: $i, j: $j }
    };
}
