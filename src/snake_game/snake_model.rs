use std::collections::VecDeque;
use std::iter;

use crate::model::grid_world::{Color, Direction, GridPoint, GridWorld, GridWorldEntity};
use crate::policy::Policy;
use rand::Rng;

#[derive(Debug)]
pub struct Apple {
  position: GridPoint,
}

impl GridWorldEntity for Apple {
  fn get_grid_cells<'a>(&'a self) -> impl Iterator<Item = &'a GridPoint> {
    iter::once(&self.position)
  }
}

#[derive(Debug)]
pub struct Snake {
  pub position: VecDeque<GridPoint>,
}

impl GridWorldEntity for Snake {
  fn get_grid_cells<'a>(&'a self) -> impl Iterator<Item = &'a GridPoint> {
    self.position.iter()
  }
}

impl Snake {
  pub fn update(&mut self, direction: Direction, hit_apple: bool) {
    let front = self.position.front().unwrap();
    self.position.push_front(front.move_in_dir(direction));
    if !hit_apple {
      self.position.pop_back();
    }
  }
}

pub struct SnakeGameModel {
  pub snake: Snake,
  pub apple: Apple,
}

impl SnakeGameModel {
  pub fn update(&mut self, policy: &mut impl Policy<SnakeGameModel, Direction>) {
    let next_dir = policy.get_action(self);
    let hit_apple = !self.snake.intersect(&self.apple).is_empty();
    dbg!(hit_apple);
    self.snake.update(next_dir, hit_apple);
    if hit_apple {
      self.apple.position.x = rand::thread_rng().gen_range(-200..200);
      self.apple.position.y = rand::thread_rng().gen_range(-200..200);
    }
  }
  pub fn new_snake_model() -> SnakeGameModel {
    let snake_color = Color { r: 255, g: 0, b: 0 };

    let apple_color = Color { r: 0, g: 128, b: 0 };
    SnakeGameModel {
      snake: Snake {
        position: vec![
          GridPoint {
            x: -4,
            y: 1,
            color: snake_color,
          },
          GridPoint {
            x: -3,
            y: 1,
            color: snake_color,
          },
          GridPoint {
            x: -2,
            y: 1,
            color: snake_color,
          },
          GridPoint {
            x: -1,
            y: 1,
            color: snake_color,
          },
          GridPoint {
            x: 0,
            y: 1,
            color: snake_color,
          },
          GridPoint {
            x: 1,
            y: 1,
            color: snake_color,
          },
        ]
        .into(),
      },
      apple: Apple {
        position: GridPoint {
          x: 10,
          y: 10,
          color: apple_color,
        },
      },
    }
  }
}

impl GridWorld for SnakeGameModel {
  fn get_grid_cells<'a>(&'a self) -> impl Iterator<Item = &'a GridPoint> {
    self
      .snake
      .get_grid_cells()
      .chain(self.apple.get_grid_cells())
  }
}
