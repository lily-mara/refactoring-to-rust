fn main() {
  let numbers = vec![1, 2, 3, 4, 5];

  let value = get_value(&numbers);

  println!("value: {}", value);
}

fn get_value() -> &i32 {
  let x = 4;
  &x
}
