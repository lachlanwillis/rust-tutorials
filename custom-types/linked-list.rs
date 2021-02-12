use crate::List::*;

enum List {
  // cons: tuple struct that wraps an element and a pointer to the next node
  Cons(u32, Box<List>),
  // Nil: a node that signifies the end of a linked list
  Nil,
}

impl List {
  // create a new empty list
  fn new() -> List {
    Nil
  }

  // consume a list and return the same list with a new element at its front
  fn prepend(self, elem: u32) -> List {
    Cons(elem, Box::new(self))
  }

  // return the length of the list
  fn len(&self) -> u32 {
    // `self` has to be matched, because the behavior of this method depends on the variant of `self`
    // `self` has type `&List`, and `*self` has type `List`, matching on a concrete type `T` is preferred over a match on a reference `&T`
    match *self {
      // can't take ownership of the tail because self is borrowed, instead take a reference to the tail
      Cons(_, ref tail) => 1 + tail.len(),
      Nil => 0,
    }
  }

  // return representation of the list as a "heap allocated" string
  fn stringify(&self) -> String {
    match *self {
      Cons(head, ref tail) => {
        // `format!` is similar to `print!`, but returns a heap allocated string instead of printing to the console
        format!("{}, {}", head, tail.stringify())
      },
      Nil => {
        format!("Nil")
      },
    }
  }
}

fn main() {
  // create an empty linked list
  let mut list = List::new();

  list = list.prepend(1);
  list = list.prepend(2);
  list = list.prepend(3);

  // show the final state of the list
  println!("linked list has a length: {}", list.len());
  println!("{}", list.stringify());
}