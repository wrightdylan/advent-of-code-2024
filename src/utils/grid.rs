use crate::prelude::*;

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

impl<T: Clone + Copy + PartialEq> Grid<T> {
    pub fn new(width: usize, height: usize, entity: Vec<T>) -> Self {
        Self { width, height, entity }
    }

    pub fn new_fill(width: usize, height: usize, fill: T) -> Self {
        let entity = vec![fill.clone(); width * height];
        Self { width, height, entity }
    }

    pub fn place_at<'a, I>(&mut self, points: I, value: T)
    where
        I: IntoIterator<Item = &'a (usize, usize)>
    {
        for &(x, y) in points {
            let index = y * self.width + x;
            if index < self.entity.len() {
                self.entity[index] = value.clone();
            }
        }
    }

    pub fn neighbours(&self, pos: &(usize, usize)) -> Vec<((usize, usize), Ortho)> {
        let mut neighbours = Vec::new();

        for (dy, dx) in &ORTHO {
            let new_y = (pos.1 as i32 + dy) as usize;
            let new_x = (pos.0 as i32 + dx) as usize;
            let en = match (dx, dy) {
                (0, 1)  => Ortho::South,
                (1, 0)  => Ortho::East,
                (0, -1) => Ortho::North,
                (-1, 0) => Ortho::West,
                _ => unreachable!(),
            };
            if new_x < self.width && new_y < self.height {
                neighbours.push(((new_x, new_y), en));
            }
        }

        neighbours
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
    pub fn draw_enum_map(&self, char_map: &HashMap<T, char>)
    where
        T: Copy + Eq + Hash,
    {
        println!("Width: {}, height: {}", self.width, self.height);
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = row * self.width + col;
                let ch = match char_map.get(&self.entity[idx]) {
                    Some(&character) => character,
                    None => '?', // Placeholder
                };
                print!("{}", ch);
            }
            println!();
        }
    }

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

impl<Char> Grid<Char>
where 
    Char: std::fmt::Debug,
{
    pub fn draw_map(&self) {
        println!("Width: {}, height: {}", self.width, self.height);
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = row * self.width + col;
                print!("{:?}", self.entity[idx]);
            }
            println!();
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