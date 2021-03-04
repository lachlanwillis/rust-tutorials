// if-else: no parentheses, each condition is followed by a block, they are expressions, all branches must return same type

fn main() {
  let n = 5;

  if n < 0 {
    print!("{} is negative", n);
  } else if n > 0 {
    print!("{} is positive", n);
  } else {
    print!("{} is zero", n);
  }

  let big_n = 
    if n < 10 && n > -10 {
      println!(", and is a small number, increases ten-fold");

      // this expression returns as i32
      10 * n
    } else {
      println!(", and is a big number, half the number");

      // this expression must return an i32 an well
      n / 2
    }; // we have a semicolon, all let bindings need it

  println!("{} -> {}", n, big_n);
}