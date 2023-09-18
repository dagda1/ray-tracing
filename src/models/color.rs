use std::ops::Add;

#[derive(Debug, PartialEq)]
pub struct Color {
  pub red: f32,
  pub green: f32,
  pub blue: f32
}

impl Color {
  pub fn new(red: f32, green: f32, blue: f32) -> Self {
    Color { red, green, blue }
  }
}

impl Add for Color {
  type Output = Self;

  fn add(self, other: Self) -> Self::Output {
    Color::new(self.red + other.red, self.green + other.green, self.blue + other.blue)
  }
}

pub fn color(red: f32, green: f32, blue: f32) -> Color {
  Color::new(red, green, blue)
} 

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_colors_are_red_green_blue_tuples() {
    let c = color(-0.5, 0.4, 1.7);

    assert_eq!(c.red, - 0.5);
    assert_eq!(c.green, 0.4);
    assert_eq!(c.blue, 1.7);
  }

  #[test]
  fn test_adding_color() {
    let c1 = color(0.9, 0.6, 0.75);
    let c2 = color(0.7, 0.1, 0.25);

    assert_eq!(c1 + c2, color(1.6, 0.7, 1.0));
  }
}