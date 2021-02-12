#![allow(dead_code)]

enum Status {
  Rich,
  Poor,
}

enum Work {
  Civilian,
  Soldier,
}

fn main() {
  // explicitly use each name so they are available without manual scoping
  use crate::Status::{Poor, Rich};
  // automatically use each name inside work
  use crate::Work::*;

  // equivalent to Status::Poor
  let status = Poor;
  // equivalent to Work::Civilian
  let work = Civilian;

  match status {
    // note the lack of scoping because of the explicit "use" above
    Rich => println!("The rich have lots of money!"),
    Poor => println!("The poor have no money..."),
  }

  match work {
    // not again the lack of scoping
    Civilian => println!("Civilians work!"),
    Soldier => println!("Soldiers fight!"),
  }
}