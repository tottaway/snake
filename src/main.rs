use std::collections::VecDeque;
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

#[derive(Debug, Copy, Clone)]
enum Direction {
  Up,
  Down,
  Left,
  Right,
}

#[derive(Debug, PartialEq)]
struct GridPoint {
  x: i32,
  y: i32,
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

#[derive(Debug)]
struct Snake {
  position: VecDeque<GridPoint>,
}

#[derive(Debug)]
struct Apple {
  position: GridPoint,
}

impl Snake {
  pub fn get_points(&self) -> Vec<(f32, f32)> {
    self
      .position
      .iter()
      .map(|point| (point.x as f32, point.y as f32))
      .collect()
  }

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
  Model {
    snake: Snake {
      position: vec![
        GridPoint { x: -4, y: 1 },
        GridPoint { x: -3, y: 1 },
        GridPoint { x: -2, y: 1 },
        GridPoint { x: -1, y: 1 },
        GridPoint { x: 0, y: 1 },
        GridPoint { x: 1, y: 1 },
      ]
      .into(),
    },
    apple: Apple {
      position: GridPoint { x: 10, y: 10 },
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

fn view(app: &App, model: &Model, frame: Frame) {
  let draw = app.draw();

  draw.background().color(LIGHTGRAY);
  for point in model.snake.get_points() {
    draw
      .rect()
      .color(RED)
      .h(10f32)
      .w(10f32)
      .x(point.0 * 10f32)
      .y(point.1 * 10f32);
  }

  draw
    .rect()
    .color(GREEN)
    .h(10f32)
    .w(10f32)
    .x(model.apple.position.x as f32 * 10f32)
    .y(model.apple.position.y as f32 * 10f32);
  draw.to_frame(app, &frame).unwrap();
}
