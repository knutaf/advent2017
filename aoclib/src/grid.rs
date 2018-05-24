use std;
use std::fmt;

#[derive(PartialEq, Debug, Clone)]
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
    #![allow(new_without_default_derive)]
    pub fn new() -> Grid<T> {
        Grid {
            grid : vec![],
            size_x : 0,
        }
    }

    pub fn from_rows(rows : Vec<Vec<T>>) -> Grid<T> {
        let mut grid = Grid::new();
        for row in rows {
           grid.add_row(row);
        }
        grid
    }

    pub fn size_x(&self) -> usize {
        self.size_x
    }

    pub fn size_y(&self) -> usize {
        self.grid.len() / self.size_x
    }

    pub fn add_row(&mut self, mut row : Vec<T>) {
        if self.grid.is_empty() {
            self.size_x = row.len();
        }

        if row.len() == self.size_x {
            self.grid.append(&mut row);
        } else {
            panic!("wrong row length. needs {}", row.len());
        }
    }

    pub fn add_row_slice(&mut self, row : &[T])
    where T : Clone {
        if self.grid.is_empty() {
            self.size_x = row.len();
        }

        if row.len() == self.size_x {
            self.grid.extend_from_slice(row);
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

    // 0, 0 is the upper left corner
    pub fn get(&self, x : usize, y : usize) -> Option<&T> {
        self.grid.get(self.index_for_location(x, y))
    }

    pub fn get_mut(&mut self, x : usize, y : usize) -> Option<&mut T> {
        let index = self.index_for_location(x, y);
        self.grid.get_mut(index)
    }

    pub fn iter(&self) -> std::slice::Iter<T> {
        self.grid.iter()
    }

    pub fn rows(&self) -> std::slice::Chunks<T> {
        self.grid.chunks(self.size_x())
    }

    pub fn enumerate(&self) -> GridIterator<T> {
        GridIterator {
            grid : self,
            iter : self.grid.iter().enumerate(),
        }
    }

    pub fn rotate_right(&self) -> Grid<T>
    where T : Clone {
        let mut output = Grid::new();

        for x in 0 .. self.size_x() {
            let mut new_row = Vec::with_capacity(self.size_y());
            for y in 0 .. self.size_y() {
                new_row.push(self.get(x, self.size_y() - y - 1).unwrap().clone());
            }
            output.add_row(new_row);
        }

        output
    }

    pub fn flip_across_y(&self) -> Grid<T>
    where T : Clone {
        let mut output = Grid::new();

        for y in 0 .. self.size_y() {
            let mut new_row = Vec::with_capacity(self.size_x());
            for x in 0 .. self.size_x() {
                new_row.push(self.get(self.size_x() - x - 1, y).unwrap().clone());
            }
            output.add_row(new_row);
        }

        output
    }

    pub fn matches_on(&self, other : &Grid<T>, offset_in_other_x : usize, offset_in_other_y : usize) -> bool
    where T : PartialEq {
        if self.size_x() + offset_in_other_x <= other.size_x() &&
           self.size_y() + offset_in_other_y <= other.size_y() {
            self.enumerate().all(|((x, y), value)| {
                value == other.get(x + offset_in_other_x, y + offset_in_other_y).unwrap()
            })
        } else {
            false
        }
    }

    pub fn stamp_onto(&self, other : &mut Grid<T>, offset_in_other_x : usize, offset_in_other_y : usize)
    where T : Clone {
        if self.size_x() + offset_in_other_x <= other.size_x() &&
           self.size_y() + offset_in_other_y <= other.size_y() {
            for ((x, y), value) in self.enumerate() {
                *other.get_mut(x + offset_in_other_x, y + offset_in_other_y).unwrap() = value.clone()
            }
        } else {
            panic!("wants to write out of bounds!");
        }
    }
}

impl<T> fmt::Display for Grid<T>
where T : fmt::Display {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut _ret = write!(f, "");
        for row in self.rows() {
            for elem in row {
                _ret = write!(f, "{}", elem);
            }
            _ret = writeln!(f);
        }
        _ret
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
        let mut grid = Grid::<u32>::new();
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
        let mut grid = Grid::<u32>::new();
        for row in 0 .. 2 {
            grid.add_row(vec![row, row + 1, row + 2, row + 3, row + 4]);
        }

        assert_eq!(grid.iter().map(|v| *v).collect::<Vec<u32>>(), vec![0, 1, 2, 3, 4, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn enumerate() {
        let mut grid = Grid::<u32>::new();
        for row in 0 .. 2 {
            grid.add_row(vec![row, row + 1]);
        }

        assert_eq!(grid.enumerate().map(|(l, v)| (l, *v)).collect::<Vec<((usize, usize), u32)>>(), vec![((0, 0), 0), ((1, 0), 1), ((0, 1), 1), ((1, 1), 2)]);
    }

    #[test]
    fn rotate_right() {
        // #..   ..#
        // ...   #..
        // .#.   ...
        let grid = Grid::<bool>::from_rows(
            vec![
                vec![true, false, false],
                vec![false, false, false],
                vec![false, true, false],]);

        let expected = Grid::<bool>::from_rows(
            vec![
                vec![false, false, true],
                vec![true, false, false],
                vec![false, false, false],]);

        assert_eq!(grid.rotate_right(), expected);
    }

    #[test]
    fn flip_across_y() {
        // #..   ..#
        // ...   ...
        // .#.   .#.
        let grid = Grid::<bool>::from_rows(
            vec![
                vec![true, false, false],
                vec![false, false, false],
                vec![false, true, false],]);

        let expected = Grid::<bool>::from_rows(
            vec![
                vec![false, false, true],
                vec![false, false, false],
                vec![false, true, false],]);

        assert_eq!(grid.flip_across_y(), expected);
    }

    #[test]
    fn matches_on_yes_1() {
        let grid = Grid::<bool>::from_rows(
            vec![
                vec![true, false],
                vec![false, false],]);

        assert_eq!(grid.matches_on(&grid.clone(), 0, 0), true);
    }

    #[test]
    fn matches_on_yes_2() {
        // #.
        // ..
        let grid = Grid::<bool>::from_rows(
            vec![
                vec![true, false],
                vec![false, false],]);

        // #..
        // ...
        // .#.
        let other = Grid::<bool>::from_rows(
            vec![
                vec![true, false, false],
                vec![false, false, false],
                vec![false, true, false],]);

        assert_eq!(grid.matches_on(&other, 0, 0), true);
    }

    #[test]
    fn matches_on_yes_3() {
        // ..
        // #.
        let grid = Grid::<bool>::from_rows(
            vec![
                vec![false, false],
                vec![true, false],]);

        // #..
        // ...
        // .#.
        let other = Grid::<bool>::from_rows(
            vec![
                vec![true, false, false],
                vec![false, false, false],
                vec![false, true, false],]);

        assert_eq!(grid.matches_on(&other, 1, 1), true);
    }

    #[test]
    fn matches_on_no_1() {
        let grid = Grid::<bool>::from_rows(
            vec![
                vec![true, false],
                vec![false, false],]);

        // #..
        // ...
        // .#.
        let other = grid.rotate_right();

        assert_eq!(grid.matches_on(&other, 0, 0), false);
    }

    #[test]
    fn matches_on_no_2() {
        // ..
        // #.
        let grid = Grid::<bool>::from_rows(
            vec![
                vec![false, false],
                vec![true, false],]);

        // #..
        // ...
        // .#.
        let other = Grid::<bool>::from_rows(
            vec![
                vec![true, false, false],
                vec![false, false, false],
                vec![false, true, false],]);

        assert_eq!(grid.matches_on(&other, 0, 1), false);
    }

    #[test]
    fn stamp_onto_1() {
        // ..
        // #.
        let grid = Grid::<bool>::from_rows(
            vec![
                vec![false, false],
                vec![true, false],]);

        // #..
        // ...
        // .#.
        let mut other = Grid::<bool>::from_rows(
            vec![
                vec![true, false, false],
                vec![false, false, false],
                vec![false, true, false],]);

        grid.stamp_onto(&mut other, 0, 0);
        assert_eq!(grid.matches_on(&other, 0, 0), true);
    }

    #[test]
    fn stamp_onto_2() {
        // ..
        // #.
        let grid = Grid::<bool>::from_rows(
            vec![
                vec![false, false],
                vec![true, false],]);

        // ..
        // ..
        let mut other = Grid::<bool>::from_rows(
            vec![
                vec![false, false],
                vec![false, false],]);

        grid.stamp_onto(&mut other, 0, 0);
        assert_eq!(grid, other);
    }

    #[test]
    fn rows() {
        let grid = Grid::<bool>::from_rows(
            vec![
                vec![true, false, false],
                vec![false, false, false],
                vec![false, true, false],]);

        assert_eq!(grid, Grid::from_rows(grid.rows().map(|a| a.iter().cloned().collect()).collect()));
    }

    #[test]
    fn add_row_slice() {
        let mut grid = Grid::<bool>::new();
        let v = vec![true, false];
        grid.add_row_slice(v.as_slice());
        grid.add_row_slice(v.as_slice());

        let other = Grid::<bool>::from_rows(
            vec![
                vec![true, false],
                vec![true, false],]);

        assert_eq!(grid, other);
    }
}
