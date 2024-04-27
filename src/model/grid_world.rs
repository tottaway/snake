use std::collections::{HashSet, VecDeque};

#[derive(Debug, Copy, Clone)]
pub enum Direction {
  Up,
  Down,
  Left,
  Right,
}

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
pub struct Color {
  pub r: u8,
  pub g: u8,
  pub b: u8,
}

#[derive(Debug, Hash, Copy, Clone)]
pub struct GridPoint {
  pub x: i32,
  pub y: i32,
  pub color: Color,
}

impl PartialEq for GridPoint {
  fn eq(&self, other: &Self) -> bool {
    self.x == other.x && self.y == other.y
  }
}

impl Eq for GridPoint {}

impl GridPoint {
  pub fn move_in_dir(&self, dir: Direction) -> GridPoint {
    match dir {
      Direction::Up => GridPoint {
        x: self.x,
        y: self.y + 1,
        color: self.color,
      },
      Direction::Left => GridPoint {
        x: self.x - 1,
        y: self.y,
        color: self.color,
      },
      Direction::Right => GridPoint {
        x: self.x + 1,
        y: self.y,
        color: self.color,
      },
      Direction::Down => GridPoint {
        x: self.x,
        y: self.y - 1,
        color: self.color,
      },
    }
  }
}

pub trait GridWorldEntity {
  fn get_grid_cells<'a>(&'a self) -> impl Iterator<Item = &'a GridPoint>;

  fn intersect(&self, other: &impl GridWorldEntity) -> VecDeque<GridPoint> {
    let first_cells: HashSet<GridPoint> = self.get_grid_cells().map(|e| e.clone()).collect();
    other
      .get_grid_cells()
      .filter(|e| first_cells.contains(e))
      .map(|e| e.clone())
      .collect()
  }
}

pub trait GridWorld {
  fn get_grid_cells<'a>(&'a self) -> impl Iterator<Item = &'a GridPoint>;
}
