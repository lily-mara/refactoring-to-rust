use day_kind::DayKind;

fn print_day_kind_message(day_kind: DayKind) {
  match day_kind {
    DayKind::Good => println!("I'm glad to hear you're having a good day!"),
    DayKind::Bad => println!("I'm sorry to hear you're having a bad day"),
  }
}

pub fn goodbye(name: &str) {
  println!("Goodbye, {}", name);
}

pub fn hello(name: &str) {
  println!("Hello, {}", name);
}
