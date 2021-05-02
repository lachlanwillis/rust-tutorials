// A lifetime is a construct the compiler, or more specifically the its borrow checker, uses to ensure all borrows are valid
// Lifetime begins when a variable is created and ends when it is destroyed
// Lifetimes and scopes are often referred to together but are not the same thing

// Lifetimes are annotated below with lines denoting the creation
// and destruction of each variable.
// `i` has the longest lifetime because its scope entirely encloses
// both `borrow1` and `borrow2`. The duration of `borrow1` compared
// to `borrow2` is irrelevant since they are disjoint.
fn main() {
    let i = 3; // Lifetime for `i` starts. ────────────────┐
               //                                          │
    {
        //                                                 │
        let borrow1 = &i; // `borrow1` lifetime starts. ──┐│
                          //                              ││
        println!("borrow1: {}", borrow1); //              ││
    } // `borrow1 ends. ──────────────────────────────────┘│
      //                                                   │
      //                                                   │
    {
        //                                                 │
        let borrow2 = &i; // `borrow2` lifetime starts. ──┐│
                          //                              ││
        println!("borrow2: {}", borrow2); //              ││
    } // `borrow2` ends. ─────────────────────────────────┘│
      //                                                   │
} // Lifetime ends.   ─────────────────────────────────────┘
