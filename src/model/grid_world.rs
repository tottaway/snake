use std::collections::{HashSet, VecDeque};

#[derive(Debug, Copy, Clone)]
pub enum Direction {
  Up,
  Down,
  Left,
  Right,
}

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
pub struct GridPoint {
  pub x: i32,
  pub y: i32,
}

impl GridPoint {
  pub fn move_in_dir(&self, dir: Direction) -> GridPoint {
    match dir {
      Direction::Up => GridPoint {
        x: self.x,
        y: self.y + 1,
      },
      Direction::Left => GridPoint {
        x: self.x - 1,
        y: self.y,
      },
      Direction::Right => GridPoint {
        x: self.x + 1,
        y: self.y,
      },
      Direction::Down => GridPoint {
        x: self.x,
        y: self.y - 1,
      },
    }
  }
}

pub trait GridWorldEntity {
  fn get_grid_cells<'a>(&'a self) -> impl Iterator<Item = &'a GridPoint>;

  fn get_points(&self) -> Vec<(f32, f32)> {
    self
      .get_grid_cells()
      .map(|point| (point.x as f32, point.y as f32))
      .collect()
  }
}

pub fn intersect(
  first_entity: &impl GridWorldEntity,
  second_entity: &impl GridWorldEntity,
) -> VecDeque<GridPoint> {
  let first_cells: HashSet<GridPoint> = first_entity.get_grid_cells().map(|e| e.clone()).collect();
  second_entity
    .get_grid_cells()
    .filter(|e| first_cells.contains(e))
    .map(|e| e.clone())
    .collect()
}
