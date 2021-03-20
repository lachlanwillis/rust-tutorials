// This enum purposely neither implements nor derives PartialEq.
// That is why comparing Foo::Bar == a fails below.
enum Foo {
    Bar,
}

fn main() {
    let a = Foo::Bar;

    // Variable a matches Foo::Bar
    // if Foo::Bar == a {
    // // ^-- this causes a compile-time error. Use `if let` instead.
    //     println!("a is foobar");
    // }

    if let Foo::Bar = a {
        println!("a is foobar");
    }
}
