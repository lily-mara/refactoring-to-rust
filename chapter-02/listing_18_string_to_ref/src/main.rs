fn print_admiration(name: &str) {
  println!("Wow, {} really makes you think.", name,);
}

fn main() {
  let value = String::new();
  print_admiration(value.as_str());
}
