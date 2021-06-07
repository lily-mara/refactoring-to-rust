use std::{fmt::Debug, marker::PhantomData, str::FromStr};

struct StringParser<T> {
  strings: Vec<String>,
  type_to_parse: PhantomData<T>,
}

impl<T> StringParser<T>
where
  T: FromStr,
  T::Err: Debug,
{
  fn new(strings: Vec<String>) -> Self {
    Self {
      strings,
      type_to_parse: PhantomData,
    }
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
