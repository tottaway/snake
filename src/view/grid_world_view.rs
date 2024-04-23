use std::marker::PhantomData;

use nannou::color::rgb::Srgb;
use nannou::prelude::*;

use crate::model::grid_world::{GridWorld, GridWorldEntity};

fn get_points(grid_world: &impl GridWorld) -> Vec<(f32, f32, Srgb<u8>)> {
  grid_world
    .get_grid_cells()
    .map(|point| {
      (
        point.x as f32,
        point.y as f32,
        Srgb {
          red: point.color.r,
          green: point.color.g,
          blue: point.color.b,
          standard: PhantomData,
        },
      )
    })
    .collect()
}

pub fn view(app: &App, model: &impl GridWorld, frame: Frame) {
  let draw = app.draw();

  draw.background().color(LIGHTGRAY);
  let points = get_points(model);
  for (x, y, color) in points {
    draw
      .rect()
      .color(color)
      .h(10f32)
      .w(10f32)
      .x(x * 10f32)
      .y(y * 10f32);
  }
  draw.to_frame(app, &frame).unwrap();
}
