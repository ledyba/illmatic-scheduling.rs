pub struct PidController {
  pub(crate) set_point: f32,
  sum: f32,
  prev: f32,
}

impl PidController {
  pub fn new(set_point: f32) -> Self {
    Self {
      set_point,
      sum: 0.0,
      prev: set_point,
    }
  }
  pub fn next(&mut self, observed: f32) -> f32 {
    let input = self.set_point - observed;
    let diff = input - self.prev;
    self.sum += (self.prev + input) / 2.0;

    let p = input * 0.7;
    let i = self.sum * 0.01;
    let d  = diff * 0.1;

    self.prev = input;
    p + i + d
  }
}