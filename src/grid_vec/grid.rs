use std::{
    fmt::Display,
    ops::{Index, IndexMut},
    str::FromStr,
};

use super::Point;

#[derive(Debug, Clone)]
pub struct Grid {
    grid: Vec<u8>,
    width: usize,
}

impl FromStr for Grid {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = Vec::with_capacity(s.len());
        let mut width = None;

        for line in s.lines() {
            let w = line.len();

            if width.is_none() {
                width = Some(w);
            }

            if width.filter(|&x| x == w).is_none() {
                panic!("all lines have to be of same length");
            }

            grid.extend(line.bytes());
        }

        Ok(Self {
            grid,
            width: width.unwrap(),
        })
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (idx, cell) in self.grid.iter().enumerate() {
            write!(f, "{}", *cell as char)?;
            if idx > 0 && (idx + 1) % self.width == 0 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

impl Index<Point> for Grid {
    type Output = u8;
    fn index(&self, index: Point) -> &Self::Output {
        &self.grid[index.i * self.width + index.j]
    }
}

impl IndexMut<Point> for Grid {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self.grid[index.i * self.width + index.j]
    }
}

impl Grid {
    pub fn find(&self, f: u8) -> Option<Point> {
        let n = self
            .grid
            .iter()
            .enumerate()
            .find(|&(_, &x)| x == f)
            .map(|x| x.0)?;

        Some(Point::new(n / self.width, n % self.width))
    }

    pub fn get(&self, p: &Point) -> Option<&u8> {
        if p.i >= self.grid.len() / self.width || p.j >= self.width {
            return None;
        }
        self.grid.get(p.i * self.width + p.j)
    }

    pub fn get_mut(&mut self, p: &Point) -> Option<&mut u8> {
        if p.i >= self.grid.len() / self.width || p.j >= self.width {
            return None;
        }
        self.grid.get_mut(p.i * self.width + p.j)
    }

    pub fn size(&self) -> (usize, usize) {
        (self.grid.len() / self.width, self.width)
    }
}
