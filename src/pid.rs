pub struct PidController {
  pub(crate) set_point: f32,
  sum: f32,
  prev: Option<f32>,
  p_gain: f32,
  i_gain: f32,
  d_gain: f32,
}

impl PidController {
  pub fn new(p_gain: f32, i_gain: f32, d_gain: f32, set_point: f32) -> Self {
    Self {
      set_point,
      sum: 0.0,
      prev: None,
      p_gain,
      i_gain,
      d_gain,
    }
  }
  pub fn next(&mut self, observed: f32) -> f32 {
    let input = self.set_point - observed;
    let diff: f32;
    if let Some(prev) = self.prev {
      diff = input - prev;
      self.sum += (prev + input) / 2.0;
    } else {
      diff = 0.0;
      self.sum += input;
    }

    let p = input * self.p_gain;
    let i = self.sum * self.i_gain;
    let d  = diff * self.d_gain;

    self.prev = Some(input);
    p + i + d
  }
  pub fn p_gain(&self) -> f32 {
    self.p_gain
  }
  pub fn change_p_gain(&mut self, p_gain: f32) {
    self.p_gain = p_gain;
  }
  pub fn i_gain(&self) -> f32 {
    self.p_gain
  }
  pub fn change_i_gain(&mut self, i_gain: f32) {
    self.i_gain = i_gain;
  }
  pub fn d_gain(&self) -> f32 {
    self.d_gain
  }
  pub fn change_d_gain(&mut self, d_gain: f32) {
    self.d_gain = d_gain;
  }
  pub fn inspect_sum(&self) -> f32 {
    self.sum
  }
}