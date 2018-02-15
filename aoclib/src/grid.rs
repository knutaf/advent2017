pub struct Grid<T> {
    grid : Vec<T>,
    size_x : usize,
}

impl<T> Grid<T> {
    pub fn new(size_x : usize) -> Grid<T> {
        Grid {
            grid : vec![],
            size_x : size_x,
        }
    }

    pub fn num_rows(&self) -> usize {
        self.grid.len() / self.size_x
    }

    pub fn add_row(&mut self, mut row : Vec<T>) {
        if row.len() == self.size_x {
            self.grid.append(&mut row);
        } else {
            panic!("wrong row length. needs {}", row.len());
        }
    }

    fn index_for_location(&self, row : usize, col : usize) -> usize {
        if col < self.size_x {
            (row * self.size_x) + col
        } else {
            panic!("column out of range! needs < {}", self.size_x);
        }
    }

    pub fn get(&self, row : usize, col : usize) -> Option<&T> {
        self.grid.get(self.index_for_location(row, col))
    }

    pub fn get_mut(&mut self, row : usize, col : usize) -> Option<&mut T> {
        let index = self.index_for_location(row, col);
        self.grid.get_mut(index)
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
}
