// variable bindings have a scope and are constrained to live inside a block
// a block is a collection of statements enclosed by braces {}
fn main() {
  // this binding lives in the main fn
  let long_lived_binding = 1;

  // this is a block and has a smaller scope than the main fn
  {
    // this binding only exists within this block
    let short_lived_binding = 2;

    println!("inner short: {}", short_lived_binding);
  }
  // end of block

  // error short_lived_binding doesnt exist in this scope
  // println!("outer short: {}", short_lived_binding);

  println!("outer long: {}", long_lived_binding);

  // also variable shadowing is allowed
  let shadowed_binding = 1;

  {
    println!("before being shadowed: {}", shadowed_binding);

    // this binding *shadows* the outer one
    let shadowed_binding = "abc";

    println!("shadowed in inner block: {}", shadowed_binding);
  }

  println!("outside inner block: {}", shadowed_binding);

  // this binding *shadows* the previous binding
  let shadowed_binding = 2;
  println!("shadowed in outer block: {}", shadowed_binding);
}