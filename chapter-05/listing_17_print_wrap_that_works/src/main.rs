macro_rules! print_wrap {
  ($wrapper_name:ident, $wrapped_name:ident, $message:expr) => {
    fn $wrapper_name() -> i32 {
      let value = $wrapped_name();

      println!("{} {}", $message, value);

      value
    }
  };
}

fn main() {
  let value = do_something_with_the_value_wrapped();

  println!("end value = {}", value);
}

print_wrap!(
  get_the_value_wrapped,
  get_the_value,
  "get_the_value returned"
);
fn get_the_value() -> i32 {
  4
}

print_wrap!(
  do_something_with_the_value_wrapped,
  do_something_with_the_value,
  "do_something_with_the_value returned"
);
fn do_something_with_the_value() -> i32 {
  let value = get_the_value_wrapped();

  value * 15
}
