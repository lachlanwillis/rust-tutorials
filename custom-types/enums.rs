// Create an `enum` to classify a web event. Note how both
// names and type information together specify the variant:
// `PageLoad != PageUnload` and `KeyPress(char) != Paste(String)`.
// Each is different and independent.
enum WebEvent {
  // An enum may be 'unit-like'
  PageLoad,
  PageUnload,
  // like tuple structs
  KeyPress(char),
  Paste(String),
  // or c-like structures
  Click {x: i64, y: i64},
}

fn inspect(event: WebEvent) {
  match event {
    WebEvent::PageLoad => println!("page loaded"),
    WebEvent::PageUnload => println!("page unloaded"),
    // destructure c form within the enum
    WebEvent::KeyPress(c) => println!("pressed {}", c),
    WebEvent::Paste(s) => println!("pasted {}", s),
    // destructure click into x and y
    WebEvent::Click {x, y} => println!("clicked at x={} and y={}", x, y),
  }
}

fn main() {
  let pressed = WebEvent::KeyPress('x');
  // to_owned() creates an owned String from a string slice
  let pasted = WebEvent::Paste("my text".to_owned());
  let click = WebEvent::Click {x: 20, y: 80};
  let load = WebEvent::PageLoad;
  let unload = WebEvent::PageUnload;

  inspect(pressed);
  inspect(pasted);
  inspect(click);
  inspect(load);
  inspect(unload);
}