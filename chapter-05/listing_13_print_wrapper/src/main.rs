fn main() {
  get_the_value_wrapped();
}

fn get_the_value() -> i32 {
  4
}

fn get_the_value_wrapped() -> i32 {
  let value = get_the_value();

  println!("get_the_value returned {}", value);

  value
}
