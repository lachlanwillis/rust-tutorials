fn main() {
  // variable binding
  let x = 5;
  
  // expression
  x;
  x + 1;
  15;

  let x = 5u32;

  // blocks are also expressions, the last returned expression of a block will be used as an assignment
  let y = {
    let x_squared = x * x;
    let x_cube = x_squared * x;

    // this is the expression that will be assigned to y
    x_cube + x_squared + x
  };

  let z = {
    // The semicolon suppresses this expression and `()` is assigned to `z`
    2 * x;
  };

  println!("x is {:?}", x);
  println!("y is {:?}", y);
  println!("z is {:?}", z);
}