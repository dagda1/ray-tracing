use std::ops::Add;
use rust_decimal::{Decimal, prelude::FromPrimitive};
use rust_decimal_macros::dec;

#[derive(Debug, PartialEq)]
pub struct Color {
  pub red: Decimal,
  pub green: Decimal,
  pub blue: Decimal
}

impl Color {
  pub fn new(red: f32, green: f32, blue: f32) -> Self {
    Color { 
      red: Decimal::from_f32(red).unwrap(),
      green: Decimal::from_f32(green).unwrap(),
      blue: Decimal::from_f32(blue).unwrap() 
    }
  }
}

impl Add for Color {
  type Output = Self;

  fn add(self, other: Self) -> Self::Output {
    Color {
      red: self.red + other.red,
      green: self.green + other.green,
      blue: self.blue + other.blue
    }
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

    assert_eq!(c.red, dec!(-0.5));
    assert_eq!(c.green, dec!(0.4));
    assert_eq!(c.blue, dec!(1.7));
  }

  #[test]
  fn test_adding_color() {
    let c1 = color(0.9, 0.6, 0.75);
    let c2 = color(0.7, 0.1, 0.25);

    assert_eq!(c1 + c2, color(1.6, 0.7, 1.0));
  }
}