fn main() {
  let mut x = String::with_capacity(10_000_000);
  for i in 0..10_000_000 {
    x.push('.');
  }

  println!("{}", x.len());
}
