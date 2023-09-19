use std::fmt::Display;
use std::ops::Add;
use std::ops::Sub;
use std::ops::Neg;
use std::ops::Mul;
use std::ops::Div;
use std::fmt;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum TupleType {
  Vector = 0,
  Point = 1
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub struct Tuple {
  pub x: f32,
  pub y: f32,
  pub z: f32,
  pub w: f32,
  pub is: TupleType
}

const ZERO_VECTOR: Tuple = Tuple { x: 0.0, y: 0.0, z: 0.0, w: 0.0, is: TupleType::Vector };

impl Tuple {
  pub fn new(x: f32, y: f32, z:f32, w: f32) -> Self {
    let is = if w == 0.0 {
                          TupleType::Vector 
                        } else {
                          TupleType::Point 
                        };

    Tuple { x, y, z, w, is }
  }

  pub fn magnitude(self) -> f32 {
    (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt()
  }

  pub fn normalise(self) -> Self {
    self / self.magnitude()
  }

  pub fn dot_product(self, other: Tuple) -> f32 {
    self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
  }

  pub fn cross_product(self, other: Tuple) -> Tuple {
    return vector(
      self.y * other.z - self.z * other.y,
      self.z * other.x - self.x * other.z,
      self.x * other.y - self.y * other.x
    );
  }
}

impl Add for Tuple {
  type Output = Self;

  fn add(self, other: Self) -> Self {
    Tuple::new(self.x + other.x, self.y + other.y, self.z + other.z, self.w + other.w)
  }
}

impl Sub for Tuple {
  type Output = Tuple;

  fn sub(self, other: Tuple) -> Tuple {
    Tuple::new(self.x - other.x, self.y - other.y, self.z - other.z, self.w - other.w)
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
    Tuple::new(self.x * scalar, self.y * scalar, self.z * scalar, self.w * scalar)
  }
}

impl Div<f32> for Tuple {
  type Output = Self;

  fn div(self, scalar: f32) -> Self::Output {
    Tuple::new(self.x / scalar, self.y / scalar, self.z / scalar, self.w / scalar)
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

      assert_eq!(a.x, 4.3);
      assert_eq!(a.y, -4.2);
      assert_eq!(a.z, 3.1);
      assert_eq!(a.w, 1.0);
      assert_eq!(a.is, TupleType::Point);
      assert_ne!(a.is , TupleType::Vector);
  }

  #[test]
  fn test_is_a_vector() {
      let a = vector(4.3, -4.2, 3.1);

      assert_eq!(a.x, 4.3);
      assert_eq!(a.y, -4.2);
      assert_eq!(a.z, 3.1);
      assert_eq!(a.w, 0.0);
      assert_eq!(a.is, TupleType::Vector);
      assert_ne!(a.is, TupleType::Point);
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
    assert_eq!(vector(1.0, 0.0, 0.0).magnitude(), 1.0);
    assert_eq!(vector(0.0, 1.0, 0.0).magnitude(), 1.0);
    assert_eq!(vector(1.0, 2.0, 3.0).magnitude(), 3.7416575);
    assert_eq!(vector(-1.0, -2.0, -3.0).magnitude(), 3.7416575);
  }

  #[test]
  fn test_normalising_vector() {
    assert_eq!(vector(4.0, 0.0, 0.0).normalise(), Tuple::new(1.0, 0.0, 0.0, 0.0));
    assert_eq!(vector(1.0, 2.0, 3.0).normalise(), Tuple::new(0.26726124, 0.5345225, 0.8017837, 0.0));

    let normalized = vector(1.0, 2.0, 3.0).normalise();
    

    assert_eq!(normalized.magnitude().round(), 1.0);
  }

  #[test]
  fn test_the_dot_product_of_two_tuples() {
    let a = vector(1.0, 2.0, 3.0);
    let b = vector(2.0, 3.0, 4.0);

    assert_eq!(a.dot_product(b), 20.0);
  }

  #[test]
  fn test_the_cross_product_of_two_vectors() {
    let a = vector(1.0, 2.0, 3.0);
    let b = vector(2.0, 3.0, 4.0);

    assert_eq!(a.cross_product(b), vector(-1.0, 2.0, -1.0));
    assert_eq!(b.cross_product(a), vector(1.0, -2.0, 1.0));
  }
}