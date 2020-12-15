fn print_admiration(name: String) {
  println!("Wow, {} really makes you think.", name);
}

fn main() {
  let value = "Artwork";
  print_admiration(value.to_string());
}
