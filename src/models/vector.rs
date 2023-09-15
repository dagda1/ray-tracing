use std::ops::Add;
use std::ops::Sub;

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
  pub fn new(x: f32, y: f32, z:f32, w: TupleType) -> Self {
    match w {
      TupleType::Vector => Tuple { x: x, y: y, z: z, w: 0.0, is: TupleType::Vector },
      TupleType::Point => Tuple { x: x, y: y, z: z, w: 1.0, is: TupleType::Point }
    }
  }

  pub fn get_type(self, other: Tuple) -> TupleType {
    if self.is == TupleType::Point || other.is == TupleType::Point {
      TupleType::Point
    } else if self.is == TupleType::Vector && other.is == TupleType::Vector {
      TupleType::Vector
    } else {
      panic!("two Points cannot be added together")
    }
  }

  pub fn negate(self) -> Tuple {
    ZERO_VECTOR - self
  }
}

impl Add for Tuple {
  type Output = Tuple;

  fn add(self, other: Tuple) -> Tuple {
    let tuple_type: TupleType = self.get_type(other);
    
    Tuple::new(self.x + other.x, self.y + other.y, self.z + other.z, tuple_type)
  }
}

impl Sub for Tuple {
  type Output = Tuple;

  fn sub(self, other: Tuple) -> Tuple {
    let tuple_type: TupleType = self.get_type(other);
    
    Tuple::new(self.x - other.x, self.y - other.y, self.z - other.z, tuple_type)
  }
}

pub fn point(x: f32, y:f32, z:f32) -> Tuple {
  Tuple::new(x, y, z, TupleType::Point)
}

pub fn vector(x: f32,  y: f32, z: f32) -> Tuple {
  Tuple::new(x, y, z, TupleType::Vector)
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
  }

  #[test]
  fn test_is_a_vector() {
      let a = vector(4.3, -4.2, 3.1);

      assert_eq!(a.x, 4.3);
      assert_eq!(a.y, -4.2);
      assert_eq!(a.z, 3.1);
      assert_eq!(a.w, 0.0);
      assert_eq!(a.is, TupleType::Vector);
  }

  #[test]
  fn test_adding_a_vector_to_a_point() {
    let a1 = point(3.0,-2.0,5.0);
    let a2 = vector(-2.0, 3.0, 1.0);

    assert_eq!(a1 + a2, Tuple::new(1.0, 1.0, 6.0, TupleType::Point))
  }

  #[test]
  fn test_subtracting_a_vector_from_a_point(){
    let a1 = point(3.0,2.0,1.0);
    let a2 = vector(5.0, 6.0, 7.0);

    assert_eq!(a1 - a2, Tuple::new(-2.0, -4.0, -6.0, TupleType::Point))
  }

  #[test]
  fn test_subtracting_2_vectors(){
    let a1 = vector(3.0,2.0,1.0);
    let a2 = vector(5.0, 6.0, 7.0);

    assert_eq!(a1 - a2, Tuple::new(-2.0, -4.0, -6.0, TupleType::Vector))
  }

  #[test]
  fn test_subtracting_a_vector_from_the_zero_vector() {
    let v = vector(1.0, -2.0, 3.0);

    assert_eq!(ZERO_VECTOR - v,  vector(-1.0, 2.0, -3.0));
  }

  #[test]
  fn test_negating_tuple() {
    let a = vector(1.0, -2.0, -3.0);

    assert_eq!(a.negate(), Tuple::new(-1.0, 2.0, 3.0, TupleType::Vector));
  }
}