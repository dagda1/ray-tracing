use std::fmt::Display;
use std::ops::Add;
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::{Decimal, prelude::FromPrimitive};
use std::ops::Sub;
use std::ops::Mul;
use partial_min_max::{min, max};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Color {
  pub red: Decimal,
  pub green: Decimal,
  pub blue: Decimal
}

pub fn decimal_to_8bit(decimal_value: Decimal) -> u8 {
  (max(0.0, min(1.0, decimal_value.to_f32().unwrap())) * 255.0).round() as u8
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

impl Sub for Color {
  type Output = Self;

  fn sub(self, other: Self) -> Self::Output {
    Color {
      red: self.red - other.red,
      green: self.green - other.green,
      blue: self.blue - other.blue
    }
  }
}

impl Mul<f32> for Color {
  type Output = Self;

  fn mul(self, scalar: f32) -> Self::Output {
    let decimal_scalar = Decimal::from_f32(scalar).unwrap();
    Color {
      red: self.red * decimal_scalar, 
      green: self.green * decimal_scalar, 
      blue: self.blue * decimal_scalar }
  }
}

impl Mul for Color {
  type Output = Self;

  fn mul(self, other: Self) -> Self::Output {
    Color {
      red: self.red * other.red,
      green: self.green * other.green,
      blue: self.blue * other.blue
    }
  }
}

impl Display for Color {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let resolved_red = decimal_to_8bit(self.red);
    let resolved_green = decimal_to_8bit(self.green);
    let resolved_blue = decimal_to_8bit(self.blue);
    
    write!(f, "{} {} {}", resolved_red, resolved_green, resolved_blue)
  }
}

pub fn color(red: f32, green: f32, blue: f32) -> Color {
  Color::new(red, green, blue)
} 

#[cfg(test)]
mod tests {
  use rust_decimal_macros::dec;

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

  #[test]
  fn test_subtracting_colors() {
    let c1 = color(0.9, 0.6, 0.75);
    let c2 = color(0.7, 0.1, 0.25);

    assert_eq!(c1 - c2, color(0.2, 0.5, 0.5));
  }

  #[test]
  fn test_multiplying_a_color_by_a_scalar() {
    let c = color(0.2, 0.3, 0.4);

    assert_eq!(c * 2.0, color(0.4, 0.6, 0.8));
  }

  #[test]
  fn test_multiplying_colors() {
    let c1 = color(1.0, 0.2, 0.4);
    let c2 = color(0.9, 1.0, 0.1);

    assert_eq!(c1 * c2, color(0.9, 0.2, 0.04));
  }

  #[test]
  fn test_to_string() {
    assert_eq!(format!("{}", color(1.0, 0.0, 0.0)), "255 0 0");
    assert_eq!(format!("{}", color(0.0, 0.5, 0.0)), "0 128 0");
    assert_eq!(format!("{}", color(-0.5, 0.0, 1.0)), "0 0 255");
  }
}