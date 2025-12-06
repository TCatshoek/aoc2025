use std::fmt::{Debug, Formatter};
use std::slice::{Chunks, Iter};
use glam::IVec2;

#[derive(Clone, Eq, PartialEq)]
pub struct World {
    buf: Vec<char>,
    pub width: usize,
    pub height: usize,
}

impl World {
    pub fn new(input: &str) -> Self {
        let mut buf = Vec::new();
        let mut width = 0;
        let mut height = 0;

        for (y, line) in input.lines().enumerate() {
            height = y + 1;
            for (x, c) in line.chars().enumerate() {
                width = x + 1;
                buf.push(c);
            }
        }

        Self {
            buf,
            width,
            height,
        }
    }

    pub fn from_size(width: usize, height: usize, val: char) -> Self {
        let buf = vec![val; width * height];
        Self {
            buf,
            width,
            height,
        }
    }

    pub fn is_in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    pub fn get(&self, x: usize, y: usize) -> Option<char> {
        match (x, y) {
            (x, y) if self.is_in_bounds(x, y) => {
                self.buf.get(x + y * self.width).copied()
            }
            _ => None
        }
    }

    pub fn get_at(&self, coords: IVec2) -> Option<char> {
        self.get(coords.x as usize, coords.y as usize)
    }

    pub fn set(&mut self, x: usize, y: usize, c: char) {
        if !self.is_in_bounds(x, y) { panic!("out of bounds") }
        self.buf[x + y * self.width] = c;
    }

    pub fn set_at(&mut self, coords: IVec2, c: char) {
        self.set(coords.x as usize, coords.y as usize, c);
    }

    pub fn iter_rows(&self) -> Chunks<'_, char> {
        self.buf.chunks(self.width)
    }

    pub fn iter_row(&self, row: usize) -> Iter<'_, char> {
        self.buf[row * self.width..row * self.width + self.width].into_iter()
    }

    pub fn iter_cols(&self) -> impl DoubleEndedIterator<Item = impl DoubleEndedIterator<Item = char> + '_> + '_ {
        (0..self.width).map(move |col| {
            (0..self.height).map(move |row| self.buf[col + row * self.width])
        })
    }

    pub fn find(&self, c: char) -> Vec<(usize, usize)> {
        self.buf.iter()
            .enumerate()
            .filter_map(|(idx, &x)| if c == x {
                Some((idx % self.width, idx / self.width))
            } else {
                None
            })
            .collect()
    }

    pub fn find_ivec2(&self, c: char) -> Vec<IVec2> {
        self.buf.iter()
            .enumerate()
            .filter_map(|(idx, &x)| if c == x {
                Some(IVec2::new((idx % self.width) as i32, (idx / self.width) as i32))
            } else {
                None
            })
            .collect()
    }
    
    pub fn directions() -> Vec<IVec2> {
        vec![
            IVec2::new(0, 1),
            IVec2::new(1, 1), 
            IVec2::new(1, 0),
            IVec2::new(1, -1),
            IVec2::new(0, -1),
            IVec2::new(-1, -1),
            IVec2::new(-1, 0),
            IVec2::new(-1, 1),
        ]
    }
}

impl Debug for World {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in self.iter_rows() {
            for c in row {
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find() {
        let mut world = World::from_size(6, 4, '.');

        world.set(0, 0, 'x');
        world.set(5, 0, 'x');
        world.set(3, 3, 'x');
        world.set(5, 3, 'x');

        let pos = world.find('x');
        assert_eq!(pos, vec![(0, 0), (5, 0), (3, 3), (5, 3)]);
    }
}