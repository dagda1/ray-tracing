use super::color::Color;

pub struct Canvas {
  pub width: usize,
  pub height: usize,
  pixels: Vec<Vec<Color>>
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
      let row = vec![Color::new(0.0, 0.0, 0.0); width];
      let pixels = vec![row.clone(); height];
      
      Canvas { width, height, pixels }
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
      self.pixels[y][x] = color;
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> &Color {
      &self.pixels[y][x]
    }
}

#[cfg(test)]
mod tests {
use super::*;

  #[test]
  fn test_creating_a_canvs() {
    let c = Canvas::new(10, 20);

    assert_eq!(c.width, 10);
    assert_eq!(c.height, 20);

    for pixel in c.pixels.iter().flatten() {
      assert_eq!(pixel, &Color::new(0.0, 0.0, 0.0));
    }
  }

  #[test]
  fn test_writing_pixels_to_a_canvas() {
    let mut c = Canvas::new(10, 20);

    let red = Color::new(1.0, 0.0, 0.0);

    c.write_pixel(2, 3, red);

    assert_eq!(c.pixel_at(2, 3), &red);
  }
}