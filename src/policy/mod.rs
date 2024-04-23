pub trait Policy<Model, Action> {
  fn get_action(&mut self, model: &Model) -> Action;
}
