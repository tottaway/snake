use std::thread::sleep;
use std::time::Duration;

use nannou::prelude::*;
use snake::model::grid_world::Direction;
use snake::snake_game::snake_policy::KeyboardPolicy;
use snake::snake_game::SnakeGameModel;
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

fn update(app: &App, model: &mut SnakeGameModel, _update: Update) {
  sleep(Duration::from_millis(100));
  let dir = if app.keys.down.contains(&Key::Left) {
    Some(Direction::Left)
  } else if app.keys.down.contains(&Key::Right) {
    Some(Direction::Right)
  } else if app.keys.down.contains(&Key::Down) {
    Some(Direction::Down)
  } else if app.keys.down.contains(&Key::Up) {
    Some(Direction::Up)
  } else {
    None
  };
  let mut policy = KeyboardPolicy {
    last_direction: Direction::Right,
    key_direction: dir,
  };
  model.update(&mut policy)
}
