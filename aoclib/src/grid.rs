use std;

pub struct Grid<T> {
    grid : Vec<T>,
    size_x : usize,
}

pub struct GridIterator<'t, T>
where T : 't {
    grid : &'t Grid<T>,
    iter : std::iter::Enumerate<std::slice::Iter<'t, T>>,
}

impl<T> Grid<T> {
    pub fn new(size_x : usize) -> Grid<T> {
        Grid {
            grid : vec![],
            size_x : size_x,
        }
    }

    pub fn size_x(&self) -> usize {
        self.size_x
    }

    pub fn size_y(&self) -> usize {
        self.grid.len() / self.size_x
    }

    pub fn add_row(&mut self, mut row : Vec<T>) {
        if row.len() == self.size_x {
            self.grid.append(&mut row);
        } else {
            panic!("wrong row length. needs {}", row.len());
        }
    }

    fn index_for_location(&self, x : usize, y : usize) -> usize {
        if x < self.size_x {
            (y * self.size_x) + x
        } else {
            panic!("column out of range! needs < {}", self.size_x);
        }
    }

    fn location_for_index(&self, index : usize) -> (usize, usize) {
        ((index % self.size_x), (index / self.size_x))
    }

    pub fn get(&self, x : usize, y : usize) -> Option<&T> {
        self.grid.get(self.index_for_location(x, y))
    }

    pub fn get_mut(&mut self, x : usize, y : usize) -> Option<&mut T> {
        let index = self.index_for_location(x, y);
        self.grid.get_mut(index)
    }

    pub fn iter<'t>(&'t self) -> std::slice::Iter<'t, T> {
        self.grid.iter()
    }

    pub fn enumerate<'t>(&'t self) -> GridIterator<'t, T> {
        GridIterator {
            grid : self,
            iter : self.grid.iter().enumerate(),
        }
    }
}

impl<'t, T> Iterator for GridIterator<'t, T>
where T : 't {
    type Item = ((usize, usize), &'t T);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(index, value)| {
            (self.grid.location_for_index(index), value)
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple() {
        let mut grid = Grid::<u32>::new(5);
        for row in 0 .. 5 {
            grid.add_row(vec![row, row + 1, row + 2, row + 3, row + 4]);
        }

        for row in 0 .. 5 {
            for col in 0 .. 5 {
                assert_eq!(*grid.get(row, col).unwrap(), (row + col) as u32);
            }
        }
    }

    #[test]
    fn iter() {
        let mut grid = Grid::<u32>::new(5);
        for row in 0 .. 2 {
            grid.add_row(vec![row, row + 1, row + 2, row + 3, row + 4]);
        }

        assert_eq!(grid.iter().map(|v| *v).collect::<Vec<u32>>(), vec![0, 1, 2, 3, 4, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn enumerate() {
        let mut grid = Grid::<u32>::new(2);
        for row in 0 .. 2 {
            grid.add_row(vec![row, row + 1]);
        }

        assert_eq!(grid.enumerate().map(|(l, v)| (l, *v)).collect::<Vec<((usize, usize), u32)>>(), vec![((0, 0), 0), ((1, 0), 1), ((0, 1), 1), ((1, 1), 2)]);
    }
}
