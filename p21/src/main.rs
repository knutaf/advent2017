#![feature(nll)]

extern crate aoclib;
use aoclib::*;
use std::fmt;

#[derive(Clone, PartialEq)]
enum PixelValue {
    On,
    Off,
}

type PixelGrid = aoclib::grid::Grid<PixelValue>;

// Initial state in the problem is always:
// .#.
// ..#
// ###
thread_local!(static INITIAL_ART : PixelGrid = PixelGrid::from_rows(
    vec![
        vec![PixelValue::Off, PixelValue::On, PixelValue::Off],
        vec![PixelValue::Off, PixelValue::Off, PixelValue::On],
        vec![PixelValue::On, PixelValue::On, PixelValue::On],]));

struct Transformation {
    input_variants : Vec<PixelGrid>,
    output : PixelGrid,
}

struct Art<'t> {
    grid : PixelGrid,
    transformations : &'t [Transformation],
}

impl PixelValue {
    fn parse(ch : char) -> PixelValue {
        if ch == '.' {
            PixelValue::Off
        } else {
            PixelValue::On
        }
    }
}

impl fmt::Display for PixelValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", if self == &PixelValue::Off { '.' } else { '#' })
    }
}

impl Transformation {
    fn parse_to_grid(grid_rep : &str) -> PixelGrid {
        let mut grid = PixelGrid::new();
        for line in grid_rep.split('/') {
            grid.add_row(line.chars().map(PixelValue::parse).collect());
        }
        grid
    }

    fn from(input : &str) -> Transformation {
        let mut sp = input.split(" => ");

        let base_input = sp.next().unwrap();
        let mut input_grid = Self::parse_to_grid(base_input);

        let mut input_variants = vec![];
        for _ in 0 .. 4 {
            let next = input_grid.rotate_right();
            if !input_variants.iter().any(|already| { *already == input_grid }) {
                input_variants.push(input_grid);
            }
            input_grid = next;
        }

        input_grid = input_grid.flip_across_y();
        for _ in 0 .. 4 {
            let next = input_grid.rotate_right();
            if !input_variants.iter().any(|already| { *already == input_grid }) {
                input_variants.push(input_grid.clone());
            }
            input_grid = next;
        }

        Transformation {
            input_variants,
            output : Self::parse_to_grid(sp.next().unwrap()),
        }
    }

    fn input_size(&self) -> usize {
        self.input_variants[0].size_x()
    }

    fn output_size(&self) -> usize {
        self.output.size_x()
    }

    fn matches_on(&self, other : &PixelGrid, offset_in_other_x : usize, offset_in_other_y : usize) -> bool {
        self.input_variants.iter().any(|variant| {
            variant.matches_on(other, offset_in_other_x, offset_in_other_y)
        })
    }
}

impl<'t> Art<'t> {
    fn new(transformations : &'t [Transformation]) -> Art<'t> {
        Art {
            grid : INITIAL_ART.with(|init| {
                init.clone()
            }),
            transformations
        }
    }

    fn num_on(&self) -> usize {
        self.grid.iter().fold(0, |sofar, item| {
            sofar + if *item == PixelValue::On { 1 } else { 0 }
        })
    }
}

impl<'t> fmt::Display for Art<'t> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.grid)
    }
}

impl<'t> Iterator for Art<'t> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        eprintln!("grid size {}", self.grid.size_x());

        let input_transform_size = if self.grid.size_x() % 2 == 0 {
            2
        } else if self.grid.size_x() % 3 == 0 {
            3
        } else {
            panic!("unsupported grid size {}", self.grid.size_x());
        };

        let output_grid_size = if input_transform_size == 2 {
            (self.grid.size_x() * 3) / 2
        } else {
            (self.grid.size_x() * 4) / 3
        };

        let row = (0 .. output_grid_size).map(|_| PixelValue::Off).collect::<Vec<PixelValue>>();
        let mut output_grid = PixelGrid::new();
        for _ in 0 .. output_grid_size {
            output_grid.add_row_slice(row.as_slice());
        }

        for base_y in 0 .. self.grid.size_y() / input_transform_size {
            for base_x in 0 .. self.grid.size_x() / input_transform_size {
                let input_x = base_x * input_transform_size;
                let input_y = base_y * input_transform_size;

                for transformation in self.transformations.iter().filter(|transformation| {
                    transformation.input_size() == input_transform_size
                }) {
                    if transformation.matches_on(&self.grid, input_x, input_y) {
                        transformation.output.stamp_onto(&mut output_grid, base_x * transformation.output_size(), base_y * transformation.output_size());
                    }
                }
            }
        }

        self.grid = output_grid;

        //let _ = eprintln!("{}", self);

        Some(self.num_on())
    }
}

fn count_pixels_on_after_iterations(input : &str, num_iterations : usize) -> usize {
    let transformations = input.lines().map(Transformation::from).collect::<Vec<Transformation>>();
    let art = Art::new(transformations.as_slice());
    art.take(num_iterations).last().unwrap()
}

fn solve_a(input : &str) -> usize {
    count_pixels_on_after_iterations(input, 5)
}

fn solve_b(input : &str) -> usize {
    count_pixels_on_after_iterations(input, 18)
}

fn main() {
    let input = read_all_stdin();
    //eprintln!("input: {}", input);

    if aoclib::should_solve_puzzle_a() {
        println!("answer: {}", solve_a(&input));
    } else {
        println!("answer: {}", solve_b(&input));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn transform_variants_1() {
        let input = "../.. => ../..";
        let transform = Transformation::from(&input);
        assert_eq!(transform.input_variants.len(), 1);
    }

    #[test]
    fn transform_variants_2() {
        let input = "##./.../..# => ../..";
        let transform = Transformation::from(&input);
        assert_eq!(transform.input_variants.len(), 8);
    }

    #[test]
    fn a_given() {
        let input =
r"../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#";
        assert_eq!(count_pixels_on_after_iterations(&input, 2), 12);
    }
}
