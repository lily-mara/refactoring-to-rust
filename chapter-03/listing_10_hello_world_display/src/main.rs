use std::fmt::{Display, Formatter};

struct Hello {}

impl Display for Hello { // <1>
  fn fmt(&self, f: &mut Formatter) -> std::fmt::Result { // <2>
    write!(f, "Hello world!") // <3>
  }
}

fn main() {
  let x = Hello {};
  println!("{}", x); // <4>
}