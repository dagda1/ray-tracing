use std::fmt::{Display, Formatter, Result};
use super::tuple::Tuple;

#[derive(Debug)]
pub struct Projectile {
  pub position: Tuple,
  pub velocity: Tuple
}

impl Display for Projectile {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "position {}, velocity {}", self.position, self.velocity)
    }
}

#[derive(Debug)]
pub struct Environment {
  pub gravity: Tuple,
  pub wind: Tuple
}

impl Display for Environment {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
      write!(f, "gravity {}, wind {}", self.gravity, self.wind)
  }
}

pub fn tick(env: &Environment, projectile: &Projectile) -> Projectile {
  let position = projectile.position + projectile.velocity;
  let velocity = projectile.velocity + env.gravity + env.wind;

  Projectile{ position, velocity}
}