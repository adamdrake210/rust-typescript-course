use std::{f64::consts::PI, fmt::Display};

use super::area::Area;

pub struct Circle {
  pub x: f64,
  pub y: f64,
  pub radius: f64
}

impl Area for Circle {
  fn area(&self) -> f64 {
    return self.radius * self.radius * PI;
  }
}

impl Display for Circle {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      return write!(f, "Circle, radius: {}, x: {}, y: {}", self.radius, self.x, self.y);
  }
}