use std::collections::VecDeque;
use std::iter;
use std::thread::sleep;
use std::time::Duration;

use nannou::prelude::*;
use rand::seq::IteratorRandom;
use rand::Rng;
use snake::model::grid_world::{Color, Direction, GridPoint, GridWorld, GridWorldEntity};
use snake::policy::Policy;
use snake::snake_game::snake_model::SnakeGameModel;
use snake::view::grid_world_view::view;

fn main() {
  nannou::app(|_| SnakeGameModel::new_snake_model())
    .loop_mode(LoopMode::Rate {
      update_interval: Duration::from_millis(1000),
    })
    .update(update)
    .simple_window(view)
    .run();
}

struct GoForwardPolicy {}

impl Policy<SnakeGameModel, Direction> for GoForwardPolicy {
  fn get_action(&mut self, model: &SnakeGameModel) -> Direction {
    Direction::Right
  }
}

struct RandomPolicy {}

impl Policy<SnakeGameModel, Direction> for RandomPolicy {
  fn get_action(&mut self, model: &SnakeGameModel) -> Direction {
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

fn update(_app: &App, model: &mut SnakeGameModel, _update: Update) {
  sleep(Duration::from_millis(100));
  let mut policy = RandomPolicy {};
  model.update(&mut policy)
}
