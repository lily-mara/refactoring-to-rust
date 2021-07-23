use input::get_name;
use output::{goodbye, hello};

mod day_kind;
mod input;
mod output;

fn main() {
  let name = get_name();

  hello(&name);
  goodbye(&name);
}
