#[derive(PartialEq, Debug)]
pub enum TupleType {
  Vector = 0,
  Point = 1
}

pub struct Tuple {
  pub x: f32,
  pub y: f32,
  pub z: f32,
  pub w: f32,
  pub is: TupleType
}


impl Tuple {
  pub fn new(x: f32, y: f32, z:f32, w: TupleType) -> Self {
    match w {
      TupleType::Vector => Tuple { x: x, y: y, z: z, w: 0.0, is: TupleType::Vector },
      TupleType::Point => Tuple { x: x, y: y, z: z, w: 1.0, is: TupleType::Point }
    }
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
}