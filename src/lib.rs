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
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;

aoc_lib! { year = 2024 }

mod utils;

pub mod prelude {
    pub use crate::{utils::*, hashset};
    pub use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
    pub use std::cmp::{Ordering, min, max};
    pub use std::fmt;
    pub use std::hash::Hash;
    pub use std::ops::{Index, IndexMut};
}

pub use prelude::*;