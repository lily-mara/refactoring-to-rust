macro_rules! identity {
  ($value:expr) => {
    $value
  };
}

fn main() {
  println!("The value is {}", identity!(4));
}
