use std::fmt::{self, Formatter, Display};

struct City {
  name: &'static str,
  lat: f32,
  lng: f32,
}

impl Display for City {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    let lat_c = if self.lat >= 0.0 {'N'} else {'S'};
    let lng_c = if self.lng >= 0.0 {'E'} else {'W'};

    write!(f, "{}: {:.3}°{} {:.3}°{}", self.name, self.lat.abs(), lat_c, self.lng.abs(), lng_c)
  }
}

#[derive(Debug)]
struct Color {
  red: u8,
  green: u8,
  blue: u8,
}

fn color_to_hex(color: &Color) -> String {
  format!("0x{:02X}{:02X}{:02X}", color.red, color.green, color.blue)
}

impl Display for Color {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "RGB ({r}, {g}, {b}) {hex}", r=self.red, g=self.green, b=self.blue, hex=color_to_hex(self))
  }
}

fn main() {
  for city in [
    City { name: "Dublin", lat: 53.347778, lng: -6.259722 },
    City { name: "Oslo", lat: 59.95, lng: 10.75 },
    City { name: "Vancouver", lat: 49.25, lng: -123.1 },
  ].iter() {
    println!("{}", *city);
  }

  for color in [
    Color { red: 128, green: 255, blue: 90 },
    Color { red: 0, green: 3, blue: 254 },
    Color { red: 0, green: 0, blue: 0 },
  ].iter() {
    println!("{}", *color);
  }
}