use std::{
    fmt::Display,
    ops::{Index, IndexMut},
    str::FromStr,
};

use super::Point;

#[derive(Debug, Clone)]
pub struct Grid {
    grid: Vec<Vec<u8>>,
}

impl FromStr for Grid {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid = s.lines().map(|x| x.as_bytes().to_vec()).collect();
        Ok(Self { grid })
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.grid.iter() {
            for cell in row.iter() {
                write!(f, "{}", *cell as char)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Index<Point> for Grid {
    type Output = u8;
    fn index(&self, index: Point) -> &Self::Output {
        &self.grid[index.i][index.j]
    }
}

impl IndexMut<Point> for Grid {
    fn index_mut(&mut self, index: Point) -> &mut Self::Output {
        &mut self.grid[index.i][index.j]
    }
}

impl Grid {
    pub fn find(&self, f: u8) -> Option<Point> {
        let (i, j, _) = self
            .grid
            .iter()
            .enumerate()
            .flat_map(|(i, x)| x.iter().copied().enumerate().map(move |(j, y)| (i, j, y)))
            .find(|x| x.2 == f)?;

        Some(Point::new(i, j))
    }

    pub fn get(&self, p: &Point) -> Option<&u8> {
        self.grid.get(p.i).and_then(|r| r.get(p.j))
    }

    pub fn get_mut(&mut self, p: &Point) -> Option<&mut u8> {
        self.grid.get_mut(p.i).and_then(|r| r.get_mut(p.j))
    }
}
