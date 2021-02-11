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
struct Point {
  x: f32,
  y: f32,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
struct Rectangle {
  top_left: Point,
  bottom_right: Point,
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
}