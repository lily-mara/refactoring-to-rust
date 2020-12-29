use std::fs::File;
use std::io::Write; // <1>

fn main() {
  let language = std::env::var("GREET_LANG").unwrap(); // <2>

  let mut file = File::create("src/greet.rs").unwrap(); // <3>
  file.write_all(language.as_bytes()).unwrap(); // <4>
}
