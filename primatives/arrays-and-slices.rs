use std::mem;

// This function borrows a slice
fn analyse_slice(slice: &[i32]) {
  println!("first element of the slice is: {}", slice[0]);
  println!("the slice has {} elements", slice.len());
}

fn main() {
  // Fixed size array (type signature is superfluous)
  let xs: [i32; 5] = [1, 2, 3, 4, 5];

  // All elements can be initialised with the same value
  let ys: [i32; 500] = [0; 500];

  println!("first element of the array is: {}", xs[0]);
  println!("secong element of the array is: {}", xs[0]);

  println!("number of elements in the array: {}", xs.len());

  // Arrays are stack allocated
  println!("array occupies {} bytes", mem::size_of_val(&xs));

  // Arrays can be automatically borrowed as slices
  println!("borrow the whole array as a slice");
  analyse_slice(&xs);

  // Slices can point to a section of an array
  // They are in the form of [starting_index..ending_index]
  // starting_index is the first position in the slice
  // ending_index is the last position in the slice
  println!("borrow a section of the array as a slice");
  analyse_slice(&ys[1 .. 4]);

  // Out of bounds indexing causes compile error
  // println!("{}", xs[5]);
}