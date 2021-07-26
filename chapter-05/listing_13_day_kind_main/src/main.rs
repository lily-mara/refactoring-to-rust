use input::{get_name, how_was_day};
use output::{goodbye, hello, print_day_kind_message};

mod day_kind;
mod input;
mod output;

fn main() {
  let name = get_name();

  hello(&name);

  let day_kind = how_was_day();
  print_day_kind_message(day_kind);

  goodbye(&name);
}
