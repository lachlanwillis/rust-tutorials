enum VeryVerboseEnumOfThingsToDoWithNumbers {
  Add,
  Subtract,
}

// This is a very common place to see type aliases, in impl functions with "self"
impl VeryVerboseEnumOfThingsToDoWithNumbers {
  fn run(&self, x: i32, y: i32) -> i32 {
    match self {
      Self::Add => x + y,
      Self::Subtract => x - y,
    }
  }
}

type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

fn main() {
  // we can refer to each variant by its alias not its long and inconvenient name
  let x = Operations::Add;

  println!("using the enum add variant: {}", x.run(2, 3));
}