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

    fn to_ppm(&self) -> String {
      let mut ppm = String::from(r#"P3
5 3
255
"#);

      for row in self.pixels.iter() {
        for &col in row.iter() {
          ppm.push_str(&format!("{}", &col));
          ppm.push_str(" ");
        }
        ppm.pop();
        ppm.push_str("\n");
      }

      ppm
    }
}

#[cfg(test)]
mod tests {
use crate::models::color::color;

use super::*;

  #[test]
  fn test_creating_a_canvs() {
    let c = Canvas::new(10, 20);

    assert_eq!(c.width, 10);
    assert_eq!(c.height, 20);

    for pixel in c.pixels.iter().flatten() {
      assert_eq!(pixel, &color(0.0, 0.0, 0.0));
    }
  }

  #[test]
  fn test_writing_pixels_to_a_canvas() {
    let mut c = Canvas::new(10, 20);

    let red = color(1.0, 0.0, 0.0);

    c.write_pixel(2, 3, red);

    assert_eq!(c.pixel_at(2, 3), &red);
  }

  #[test]
  fn test_constructing_the_ppm_pixel_data(){
    let mut c = Canvas::new(5, 3);

    let c1 = color(1.5, 0.0, 0.0);
    let c2 = color(0.0, 0.5, 0.0);
    let c3 = color(-0.5, 0.0, 1.0);

    c.write_pixel(0, 0, c1);
    c.write_pixel(2, 1, c2);
    c.write_pixel(4, 2, c3);

    let mut ppm = c.to_ppm()
                .lines()
                .skip(3)
                .take(3)
                .collect::<Vec<&str>>()
                .join("\n");

    ppm.push_str("\n");

    assert_eq!(ppm, r#"255 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 128 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 255
"#)
  }
}