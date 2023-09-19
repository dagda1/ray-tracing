use std::fmt::Display;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Neg;
use std::ops::Mul;
use std::ops::Div;
use std::fmt;
use rust_decimal::{Decimal, prelude::FromPrimitive,MathematicalOps};
use rust_decimal_macros::dec;
use num_traits::ToPrimitive;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum TupleType {
  Vector = 0,
  Point = 1
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Tuple {
  pub x: Decimal,
  pub y: Decimal,
  pub z: Decimal,
  pub w: Decimal,
}

const ZERO_VECTOR: Tuple = Tuple { x: dec!(0.0), y: dec!(0.0), z: dec!(0.0), w: dec!(0.0) };

impl Tuple {
  pub fn new(x: f32, y: f32, z:f32, w: f32) -> Self {
    Tuple {
      x: Decimal::from_f32(x).unwrap(),
      y: Decimal::from_f32(y).unwrap(),
      z: Decimal::from_f32(z).unwrap(), 
      w: Decimal::from_f32(w).unwrap(),
    }
  }

  pub fn is(self) -> TupleType {
    if self.w == dec!(0.0) {
      TupleType::Vector 
    } else {
      TupleType::Point 
    }
  }

  pub fn magnitude(self) -> Decimal {
    let mag = (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt();

    match mag {
      Some(val) => val,
      None => panic!("A square root cannot have a negative")
    }
  }

  pub fn normalise(self) -> Self {
    self / self.magnitude().to_f32().unwrap()
  }

  pub fn dot_product(self, other: Tuple) -> Decimal {
    self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
  }

  pub fn cross_product(self, other: Tuple) -> Tuple {
    return vector(
      (self.y * other.z - self.z * other.y).to_f32().unwrap(),
      (self.z * other.x - self.x * other.z).to_f32().unwrap(),
      (self.x * other.y - self.y * other.x).to_f32().unwrap()
    );
  }
}

impl Add for Tuple {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    Tuple { 
      x:self.x + other.x,
      y: self.y + other.y,
      z: self.z + other.z,
      w: self.w + other.w }
  }
}

impl Sub for Tuple {
  type Output = Tuple;

  fn sub(self, other: Tuple) -> Tuple {
    Tuple { 
      x: self.x - other.x,
      y: self.y - other.y,
      z: self.z - other.z, 
      w: self.w - other.w
    }
  }
}

impl Neg for Tuple {
  type Output = Tuple;

  fn neg(self) -> Self::Output {
    ZERO_VECTOR - self
  }
}

impl Mul<f32> for Tuple {
  type Output = Self;

  fn mul(self, scalar: f32) -> Self::Output {
    let scalar_decimal = Decimal::from_f32(scalar).unwrap();
    Tuple {
      x: self.x * scalar_decimal,
      y: self.y * scalar_decimal,
      z: self.z * scalar_decimal,
      w: self.w * scalar_decimal
    }
  }
}

impl Div<f32> for Tuple {
  type Output = Self;

  fn div(self, scalar: f32) -> Self::Output {
let scalar_decimal = Decimal::from_f32(scalar).unwrap();
    
    Tuple {
      x: self.x / scalar_decimal,
      y: self.y / scalar_decimal,
      z: self.z / scalar_decimal,
      w: self.w / scalar_decimal
    }
  }
}

impl Display for Tuple {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({}, {}, {})", self.x, self.y, self.z)
  }
}

pub fn point(x: f32, y:f32, z:f32) -> Tuple {
  Tuple::new(x, y, z, 1.0)
}

pub fn vector(x: f32,  y: f32, z: f32) -> Tuple {
  Tuple::new(x, y, z, 0.0)
}

#[cfg(test)]
mod tests {
use super::*;

  #[test]
  fn test_is_a_point() {
      let a = point(4.3, -4.2, 3.1);

      assert_eq!(a.x, dec!(4.3));
      assert_eq!(a.y, dec!(-4.2));
      assert_eq!(a.z, dec!(3.1));
      assert_eq!(a.w, dec!(1.0));
      assert_eq!(a.is(), TupleType::Point);
      assert_ne!(a.is() , TupleType::Vector);
  }

  #[test]
  fn test_is_a_vector() {
      let a = vector(4.3, -4.2, 3.1);

      assert_eq!(a.x, dec!(4.3));
      assert_eq!(a.y, dec!(-4.2));
      assert_eq!(a.z, dec!(3.1));
      assert_eq!(a.w, dec!(0.0));
      assert_eq!(a.is(), TupleType::Vector);
      assert_ne!(a.is(), TupleType::Point);
  }

  #[test]
  fn test_adding_two_tuples() {
    let a1 = Tuple::new(3.0,-2.0,5.0, 1.0);
    let a2 = vector(-2.0, 3.0, 1.0);

    assert_eq!(a1 + a2, Tuple::new(1.0, 1.0, 6.0, 1.0))
  }

  #[test]
  fn test_subtracting_2_points() {
    let p1 = point(3.0, 2.0, 1.0);
    let p2 = point(5.0, 6.0, 7.0);

    assert_eq!(p1 - p2, vector(-2.0, -4.0, -6.0));
  }

  #[test]
  fn test_subtracting_a_vector_from_a_point() {
    let p = point(3.0, 2.0, 1.0);
    let v = vector(5.0, 6.0, 7.0);

    assert_eq!(p -v, point(-2.0, -4.0, -6.0)); 
  }

  #[test]
  fn test_subtracting_a_vector_from_the_zero_vector() {
    let v = vector(1.0, -2.0, 3.0);

    assert_eq!(ZERO_VECTOR - v,  vector(-1.0, 2.0, -3.0));
  }

  #[test]
  fn test_negating_tuple() {
    let a = Tuple::new(1.0, -2.0, -3.0, -4.0);

    assert_eq!(-a, Tuple::new(-1.0, 2.0, 3.0, 4.0));
  }

  #[test]
  fn test_multiplying_a_tuple_by_a_scalar() {
    let a = Tuple::new(1.0, -2.0, 3.0, -4.0);

    assert_eq!(a * 3.5, Tuple::new(3.5, -7.0, 10.5, -14.0));
  }

  #[test]
  fn test_multiplying_a_tuple_by_a_fraction() {
    let a = Tuple::new(1.0, -2.0, 3.0, -4.0);

    assert_eq!(a * 0.5, Tuple::new(0.5, -1.0, 1.5, -2.0));
  }

  #[test]
  fn test_dividing_a_tuple_by_a_scalar() {
    let a = Tuple::new(1.0, -2.0, 3.0, -4.0);

    assert_eq!(a / 2.0, Tuple::new(0.5, -1.0, 1.5, -2.0));
  }

  #[test]
  fn test_computing_the_magnitude_of_a_vector() {
    assert_eq!(vector(1.0, 0.0, 0.0).magnitude(), dec!(1.0));
    assert_eq!(vector(0.0, 1.0, 0.0).magnitude(), dec!(1.0));
    assert_eq!(vector(1.0, 2.0, 3.0).magnitude(), dec!(3.7416575));
    assert_eq!(vector(-1.0, -2.0, -3.0).magnitude(), dec!(3.7416575));
  }

  #[test]
  fn test_normalising_vector() {
    assert_eq!(vector(4.0, 0.0, 0.0).normalise(), Tuple::new(1.0, 0.0, 0.0, 0.0));
    assert_eq!(vector(1.0, 2.0, 3.0).normalise(), Tuple::new(0.26726124, 0.5345225, 0.8017837, 0.0));

    let normalized = vector(1.0, 2.0, 3.0).normalise();

    assert_eq!(normalized.magnitude(), dec!(1.0));
  }

  #[test]
  fn test_the_dot_product_of_two_tuples() {
    let a = vector(1.0, 2.0, 3.0);
    let b = vector(2.0, 3.0, 4.0);

    assert_eq!(a.dot_product(b), dec!(20.0));
  }

  #[test]
  fn test_the_cross_product_of_two_vectors() {
    let a = vector(1.0, 2.0, 3.0);
    let b = vector(2.0, 3.0, 4.0);

    assert_eq!(a.cross_product(b), vector(-1.0, 2.0, -1.0));
    assert_eq!(b.cross_product(a), vector(1.0, -2.0, 1.0));
  }
}