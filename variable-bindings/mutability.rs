// variable bindings are immutable by default but can be made mutable with the mut modifier
fn main() {
  let _immutable_variable = 1;
  let mut mutable_variable = 1;

  println!("before mutation: {}", mutable_variable);

  // this is okay
  mutable_variable += 1;

  println!("after mutation: {}", mutable_variable);

  // error this is not okay
  // _immutable_variable += 1;
}