pub fn add(a: i16, b: i16) -> i16 {
  a + b
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_add() {
      assert_eq!(add(2, 3), 5);
      assert_eq!(add(-2, 2), 0);
  }
}