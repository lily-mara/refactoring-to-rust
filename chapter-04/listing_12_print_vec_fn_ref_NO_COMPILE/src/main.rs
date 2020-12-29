fn main() {
  let numbers = vec![1, 2, 3, 4, 5];

  let value = get_value(&numbers, "Getting the number");

  println!("value: {}", value);
}

fn get_value(numbers: &Vec<i32>, s: &str) -> &i32 {
  println!("{}", s);
  &numbers[0]
}
