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

    // 1D gridness
    pub struct Grid<T> {
        pub width: usize,
        pub height: usize,
        pub entity: Vec<T>,
    }

    impl<T> Grid<T> {
        pub fn new(width: usize, height: usize, entity: Vec<T>) -> Self {
            Self { width, height, entity }
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
    
    impl<T> IndexMut<(usize, usize)> for Grid<T> {
        // Changes the element at location on canvas[(x, y)]
        fn index_mut(&mut self, (col, row): (usize, usize)) -> &mut T {
            let idx = (self.width * row) + col;
            &mut self.entity[idx]
        }
    }
}

pub use prelude::*;