// type statment can be used to give a new name/alias to an existing type
// must be UpperCamelCase except for primitive types i.e. usize, u32, etc

type Nanosecond = u64;
type Inch = u64;

#[allow(non_camel_case_types)]
type u64_t = u64;

fn main() {
  // `NanoSecond` = `Inch` = `u64_t` = `u64`.
  let nanoseconds: Nanosecond = 5 as u64_t;
  let inches: Inch = 2 as u64_t;

  // Note that type aliases *don't* provide any extra type safety, because aliases are *not* new types
  println!("{} nanoseconds + {} inches = {} unit?", nanoseconds, inches, nanoseconds + inches);
}