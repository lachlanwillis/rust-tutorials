#[derive(Debug)]
struct Person {
  name: String,
  age: u8,
}

// A unit struct
// Fieldless, useful for generics
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with 2 fields
#[derive(Debug)]
struct Point {
  x: f32,
  y: f32,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
  top_left: Point,
  bottom_right: Point,
}

fn rect_area(rect: Rectangle) -> f32 {
  let Rectangle {top_left: Point {x: top_left_x, y: top_left_y}, bottom_right: Point {x: bottom_right_x, y: bottom_right_y}} = rect;
  let length = bottom_right_x - top_left_x;
  let width = top_left_y - bottom_right_y;
  let area = length * width;

  println!("rect: {:?}", rect);
  println!("length: {}", length);
  println!("width: {}", width);

  area
}

fn square(bottom_left: Point, side: f32) -> Rectangle {
  Rectangle {top_left: Point {x: bottom_left.x, y: bottom_left.y + side}, bottom_right: Point {x: bottom_left.x + side, y: bottom_left.y}}
}

fn main() {
  // Create struct with field init shorthand
  let name = String::from("Peter");
  let age = 27;
  let peter = Person {name, age};

  // Print debug struct
  println!("{:?}", peter);

  // Instantiate a Point
  let point: Point = Point {x: 10.3, y: 0.4};

  // Access the fields of a Point
  println!("Point coords: ({}, {})", point.x, point.y);

  // Make a new Point by using struct update syntax to use the fields of our other Point
  let bottom_right = Point {x: 5.2, ..point};

  // bottom_right.y will be equal to point.y because we used that field from point
  println!("Second point: ({}, {})", bottom_right.x, bottom_right.y);

  // Destructure the Point using the let binding
  let Point {x: top_edge, y: left_edge} = point;

  let _rectangle = Rectangle {
    // struct instantiation is an expression too
    top_left: Point {x: left_edge, y: top_edge},
    bottom_right: bottom_right,
  };

  // Instantiate a Unit struct
  let _unit = Unit;

  // Instantiate a tuple struct
  let pair = Pair(1, 0.1);

  // Access the fields of a tuple struct
  println!("Pair contains {:?} and {:?}", pair.0, pair.1);

  // Destructure a tuple struct
  let Pair(integer, decimal) = pair;

  println!("Pair contains {:?} and {:?}", integer, decimal);

  println!("rect area: {}", rect_area(_rectangle));

  let sq = square(Point {x: 2.0, y: 4.0}, 3.0);
  println!("square area: {}", rect_area(sq));
}