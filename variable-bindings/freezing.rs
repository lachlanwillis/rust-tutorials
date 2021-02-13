// when data is bound by the same name immutably it also freezes
// frozen data cannot be modified until the immutable binding goes out of scope
fn main() {
  let mut _mutable_integer = 7i32;

  {
    // shadowing by immutable _mutable_integer
    let _mutable_integer = _mutable_integer;

    // error _mutable_integer is frozen in this scope and cannot be mutated
    // _mutable_integer = 50;

    // _mutable_integer goes out of scope
  }

  // _mutable_integer is not frozen in this scope
  _mutable_integer = 3;
}