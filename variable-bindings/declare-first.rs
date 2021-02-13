// it is possible to declare variables first then initialise their value later
// this is seldom used as it may lead to uninitialised variables
fn main() {
  let a_binding;

  {
    let x = 2;

    // initiaise the binding
    a_binding = x * x;
  }

  println!("a binding: {}", a_binding);

  let another_binding;

  // error use of uninitialised binding
  // println!("another binding: {}", another_binding);

  another_binding = 1;

  println!("another binding: {}", another_binding);
}