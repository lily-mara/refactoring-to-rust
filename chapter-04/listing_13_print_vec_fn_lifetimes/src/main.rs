fn main() {
  let numbers = vec![1, 2, 3, 4, 5];

  let value = get_value(&numbers, "Getting the number");

  println!("value: {}", value);
}

fn get_value<'a>(numbers: &'a Vec<i32>, s: &str) -> &'a i32 {
  println!("{}", s);
  &numbers[0]
}
