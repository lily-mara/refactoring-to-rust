macro_rules! print_wrap {
  ($wrapper_name:ident, $wrapped_name:ident, $message:expr) => {
    fn $wrapper_name() -> i32 {
      $wrapped_name()
    }
  };
}

fn main() {
  println!("get_the_value() -> {}", get_the_value());
  println!("get_the_value_wrapped() -> {}", get_the_value_wrapped());
}

print_wrap!(
  get_the_value_wrapped,
  get_the_value,
  "get_the_value returned"
);
fn get_the_value() -> i32 {
  4
}
