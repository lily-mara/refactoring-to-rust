use crate::day_kind::DayKind;
use std::io::stdin;

pub fn get_name() -> String {
  let mut name = String::new();

  println!("Please enter your name");
  stdin().read_line(&mut name).unwrap();

  name
}

pub fn how_was_day() -> DayKind {
  let mut day = String::new();

  println!("How was your day?");
  stdin().read_line(&mut day).unwrap();

  let day_trimmed = day.trim();

  if day_trimmed == "good" {
    DayKind::Good
  } else {
    DayKind::Bad
  }
}
