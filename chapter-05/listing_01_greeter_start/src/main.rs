use std::io::stdin;

fn main() {
  let name = get_name();

  hello(&name);
  goodbye(&name);
}

fn get_name() -> String {
  let mut name = String::new();

  println!("Please enter your name");
  stdin().read_line(&mut name).unwrap();

  name
}

fn goodbye(name: &str) {
  println!("Goodbye, {}", name);
}

fn hello(name: &str) {
  println!("Hello, {}", name);
}
