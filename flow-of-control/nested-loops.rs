// it is possible to break or continue outer loops when dealing with nested loops
// to do this, the loops must be annotated with some 'label and must be passed to the break or continue
#![allow(unreachable_code)]

fn main() {
  'outer: loop {
    println!("entered the outer loop");

    'inner: loop {
      println!("entered the inner loop");

      // this would break for only the inner loop
      //break;

      // this breaks the outer loop
      break 'outer;
    }

    println!("this point will never be reached");
  }

  println!("exited the outer loop");
}