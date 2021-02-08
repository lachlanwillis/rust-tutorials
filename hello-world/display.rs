// Import fmt via `use`
use std::fmt;

struct Structure(i32);

// To use `{}` marker, we need to manually implement the `fmt::Display` trait for the struct type
impl fmt::Display for Structure {
  // This trait requires fmt to be implemented with this exact signature
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "min: {}, max: {}", self.0, self.1)
  }
}

struct Point2D {
  x: f64,
  y: f64
}

impl fmt::Display for Point2D {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "x: {}, y: {}", self.x, self.y)
  }
}

impl fmt::Debug for Point2D {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Complex: x: {}, y: {}", self.x, self.y)
  }
}

fn main() {
  println!("{} is a formatted struct!!!", Structure(2));

  let minmax = MinMax(0, 14);

  println!("Compare structures:");
  println!("Display: {}", minmax);
  println!("Debug: {:?}", minmax);

  let big_range = MinMax(-300, 300);
  let small_range = MinMax(-3, 3);

  println!("The big range is {big} and the small range is {small}", big=big_range, small=small_range);
  
  let point = Point2D { x: 3.3, y: 7.2 };

  println!("Compare points:");
  println!("Display: {}", point);
  println!("Debug: {:?}", point);
  // println!("Binary: {:b}", point);
}