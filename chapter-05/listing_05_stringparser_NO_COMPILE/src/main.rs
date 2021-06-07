use std::{fmt::Debug, str::FromStr};

struct StringParser {
  strings: Vec<String>,
}

impl<T> StringParser
where
  T: FromStr,
  T::Err: Debug,
{
  fn new(strings: Vec<String>) -> Self {
    Self { strings }
  }

  fn next(&mut self) -> Option<T> {
    Some(self.strings.pop()?.parse().unwrap())
  }
}

fn main() {
  let mut parser = StringParser::<i32>::new(vec![
    "3".to_string(),
    "-1284".to_string(),
    "8".to_string(),
  ]);

  println!("{:?}", parser.next());
  println!("{:?}", parser.next());
  println!("{:?}", parser.next());
  println!("{:?}", parser.next());
}
