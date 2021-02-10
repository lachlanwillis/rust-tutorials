fn main() {
  // all vars prefixed with "_" as they are not used
  
  // type annotated
  let _logical: bool = true;

  let _number: f64 = 3.0; // regular annotation
  let _num = 5i32; // suffix annotation

  // Or a default will be used.
  let _default_float = 3.0; // `f64`
  let _default_integer = 7; // `i32`

  // Type can also be inferred from context
  let mut _inferred_type = 12; // by itself would be i32 be default
  _inferred_type = 213432594724923i64; // but instead it is inferred from the i64 on this line

  // mutable values (mut) allow the value to be "mutated/changed"
  let mut _mutable = 12;
  _mutable = 21;

  // But the type cannot be changed
  // mutable = true; // ERROR

  // variable can be overwritten with shadowing
  let _mutable = true;
}