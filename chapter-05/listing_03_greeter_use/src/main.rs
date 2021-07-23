use input::get_name;
use output::{goodbye, hello};

fn main() {
  let name = get_name();

  hello(&name);
  goodbye(&name);
}

mod input {
  use std::io::stdin;

  fn get_name() -> String {
    let mut name = String::new();

    println!("Please enter your name");
    stdin().read_line(&mut name).unwrap();

    name
  }
}

mod output {
  fn goodbye(name: &str) {
    println!("Goodbye, {}", name);
  }

  fn hello(name: &str) {
    println!("Hello, {}", name);
  }
}
