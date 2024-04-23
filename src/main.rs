use snake::model::grid_world::{Color, Direction, GridPoint, GridWorld, GridWorldEntity};
use snake::view::grid_world_view::view;
use std::collections::VecDeque;
use std::iter;
use std::thread::sleep;
use std::time::Duration;

use nannou::prelude::*;
use rand::seq::IteratorRandom;
use rand::Rng;

fn main() {
  nannou::app(model)
    .loop_mode(LoopMode::Rate {
      update_interval: Duration::from_millis(1000),
    })
    .update(update)
    .simple_window(view)
    .run();
}

#[derive(Debug)]
struct Apple {
  position: GridPoint,
}

impl GridWorldEntity for Apple {
  fn get_grid_cells<'a>(&'a self) -> impl Iterator<Item = &'a GridPoint> {
    iter::once(&self.position)
  }
}

#[derive(Debug)]
struct Snake {
  position: VecDeque<GridPoint>,
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

struct Model {
  snake: Snake,
  apple: Apple,
}

impl GridWorld for Model {
  fn get_grid_cells<'a>(&'a self) -> impl Iterator<Item = &'a GridPoint> {
    self
      .snake
      .get_grid_cells()
      .chain(self.apple.get_grid_cells())
  }
}

trait Policy {
  fn next_dir(&mut self, model: &Model) -> Direction;
}

struct GoForwardPolicy {}

impl Policy for GoForwardPolicy {
  fn next_dir(&mut self, model: &Model) -> Direction {
    Direction::Right
  }
}

struct RandomPolicy {}

impl Policy for RandomPolicy {
  fn next_dir(&mut self, model: &Model) -> Direction {
    let snake = &model.snake;
    let head = &snake.position.front().unwrap();
    loop {
      let dir = **vec![
        Direction::Up,
        Direction::Left,
        Direction::Right,
        Direction::Down,
      ]
      .iter()
      .choose_multiple(&mut rand::thread_rng(), 1)
      .first()
      .unwrap();

      if !model.snake.position.contains(&head.move_in_dir(dir)) {
        return dir;
      }
    }
  }
}

fn model(_app: &App) -> Model {
  let snake_color = Color { r: 255, g: 0, b: 0 };

  let apple_color = Color { r: 0, g: 128, b: 0 };
  Model {
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

fn update(_app: &App, model: &mut Model, update: Update) {
  sleep(Duration::from_millis(100));
  let mut policy = RandomPolicy {};
  let next_dir = policy.next_dir(model);
  let hit_apple = *model.snake.position.front().unwrap() == model.apple.position;
  model.snake.update(next_dir, hit_apple);
  if hit_apple {
    model.apple.position.x = rand::thread_rng().gen_range(-200..200);
    model.apple.position.y = rand::thread_rng().gen_range(-200..200);
  }
}
