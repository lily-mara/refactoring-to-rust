fn main() {
  let mut x = String::new();

  for i in 0..10_000_000 {
    x.push('.');
  }

  println!("{}", x.len());
}
