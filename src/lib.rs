extern crate aoc_runner;

#[macro_use]
extern crate aoc_runner_derive;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;

aoc_lib! { year = 2024 }

pub mod prelude {
    pub use std::collections::{HashMap, HashSet, VecDeque};
    use std::ops::{Index, IndexMut};

    // Orthogonals
    pub const ORTHO: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    // Cardinals and ordinals
    pub const CANDO: [(i32, i32); 8] = [(0, 1), (1, 0), (0, -1), (-1, 0), (1, 1), (-1, -1), (1, -1), (-1, 1)];

    #[derive(Debug)]
    pub struct Point {
        pub x: i32,
        pub y: i32,
    }

    impl Point {
        pub fn new(x: i32, y: i32) -> Self {
            Self { x, y }
        }
    }

    pub trait Boundary<T> {
        fn in_bounds(&self, boundary: T) -> bool;
    }

    impl Boundary<(i32, i32)> for Point {
        fn in_bounds(&self, boundary: (i32, i32)) -> bool {
            self.x >= 0 && self.x < boundary.0 &&
            self.y >= 0 && self.y < boundary.1
        }
    }

    impl Boundary<(usize, usize)> for Point {
        fn in_bounds(&self, boundary: (usize, usize)) -> bool {
            (self.x as usize).lt(&boundary.0) && (self.y as usize).lt(&boundary.1)
        }
    }

    pub enum GridError {
        OutOfBounds,
        Collision,
    }

    // 1D gridness
    pub struct Grid<T> {
        pub width: usize,
        pub height: usize,
        pub entity: Vec<T>,
    }

    impl<T: Copy + PartialEq> Grid<T> {
        pub fn new(width: usize, height: usize, entity: Vec<T>) -> Self {
            Self { width, height, entity }
        }

        pub fn peek(&self, from: &(usize, usize), dir: &(i32, i32)) -> Result<T, GridError> {
            let (from_x, from_y) = from;
            let (dir_x, dir_y) = dir;
    
            let to_x = *from_x as i32 + dir_x;
            let to_y = *from_y as i32 + dir_y;
    
            if to_x < 0 || to_x >= self.width as i32 || to_y < 0 || to_y >= self.height as i32 {
                return Err(GridError::OutOfBounds);
            }
    
            let to_idx = (to_y as usize * self.width + to_x as usize) as usize;
            Ok(self.entity[to_idx])
        }

        pub fn slide(&mut self, from: (usize, usize), dir: (i32, i32), ignore: Option<T>) -> Result<(), GridError> {    
            let to_x = from.0 as i32 + dir.0;
            let to_y = from.1 as i32 + dir.1;
    
            if to_x < 0 || to_x >= self.width as i32 || to_y < 0 || to_y >= self.height as i32 {
                return Err(GridError::OutOfBounds);
            }
    
            let from_idx = (from.1 * self.width + from.0) as usize;
            let to_idx = (to_y as usize * self.width + to_x as usize) as usize;
    
            let from_tile = self.entity[from_idx];
            let to_tile = self.entity[to_idx];
    
            if from_tile == ignore.unwrap_or(to_tile) || to_tile == ignore.unwrap_or(from_tile) {
                self.entity.swap(from_idx, to_idx);
                return Ok(());
            } else {
                return Err(GridError::Collision);
            }
        }
    }

    impl<T> Grid<T>
    where T: std::fmt::Debug {
        pub fn dump_raw(&self) {
            println!("Width: {}, height: {}", self.width, self.height);
            for row in 0..self.height {
                let start_idx = row * self.width;
                let end_idx = start_idx + self.width;
                let row_slice = &self.entity[start_idx..end_idx];
                println!("{:?}", row_slice);
            }
        }
    }

    impl<T> Index<(i32, i32)> for Grid<T> {
        type Output = T;
    
        // Returns the element at location on grid[(x, y)]
        fn index(&self, (col, row): (i32, i32)) -> &Self::Output {
            let idx = (self.width * row as usize) + col as usize;
            &self.entity[idx]
        }
    }

    impl<T> Index<(usize, usize)> for Grid<T> {
        type Output = T;
    
        // Returns the element at location on grid[(x, y)]
        fn index(&self, (col, row): (usize, usize)) -> &Self::Output {
            let idx = (self.width * row) + col;
            &self.entity[idx]
        }
    }
    
    impl<T> IndexMut<(i32, i32)> for Grid<T> {
        // Changes the element at location on grid[(x, y)]
        fn index_mut(&mut self, (col, row): (i32, i32)) -> &mut T {
            let idx = (self.width * row as usize) + col as usize;
            &mut self.entity[idx]
        }
    }

    impl<T> IndexMut<(usize, usize)> for Grid<T> {
        // Changes the element at location on grid[(x, y)]
        fn index_mut(&mut self, (col, row): (usize, usize)) -> &mut T {
            let idx = (self.width * row) + col;
            &mut self.entity[idx]
        }
    }
}

pub use prelude::*;