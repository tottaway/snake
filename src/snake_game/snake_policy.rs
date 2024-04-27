use super::snake_model::SnakeGameModel;
use crate::model::grid_world::Direction;
use crate::policy::Policy;
use rand::seq::IteratorRandom;

struct GoForwardPolicy {}

impl Policy<SnakeGameModel, Direction> for GoForwardPolicy {
  fn get_action(&mut self, _model: &SnakeGameModel) -> Direction {
    Direction::Right
  }
}

pub struct RandomPolicy {}

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

pub struct KeyboardPolicy {
  pub last_direction: Direction,
  pub key_direction: Option<Direction>,
}

impl Policy<SnakeGameModel, Direction> for KeyboardPolicy {
  fn get_action(&mut self, _model: &SnakeGameModel) -> Direction {
    self.key_direction.unwrap_or(self.last_direction)
  }
}
