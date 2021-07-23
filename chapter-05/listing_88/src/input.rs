use std::io::stdin;

pub enum DayKind {
  Good,
  Bad,
}

fn line() -> String {
  let mut line = String::new();

  stdin().read_line(&mut line).unwrap();

  line
}

pub fn get_name() -> String {
  println!("Please enter your name");

  line()
}

pub fn how_was_day() -> DayKind {
  let mut day = String::new();

  while !(day == "good\n" || day == "bad\n") {
    println!("How was your day?");
    day = line();
  }

  if day == "good\n" {
    DayKind::Good
  } else {
    DayKind::Bad
  }
}
