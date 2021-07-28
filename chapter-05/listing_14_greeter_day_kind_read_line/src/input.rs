use crate::day_kind::DayKind;
use std::io::stdin;

fn read_line() -> String {
  let mut line = String::new();

  stdin().read_line(&mut line).unwrap();

  line.trim().to_string()
}

pub fn get_name() -> String {
  println!("Please enter your name");
  read_line()
}

pub fn how_was_day() -> DayKind {
  println!("How was your day?");
  let day = read_line();

  if day == "good" {
    DayKind::Good
  } else {
    DayKind::Bad
  }
}
