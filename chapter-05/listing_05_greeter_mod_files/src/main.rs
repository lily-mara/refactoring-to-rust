use crate::input::get_name;
use crate::output::{goodbye, hello};

mod input;
mod output;

fn main() {
  let name = get_name();

  hello(&name);
  goodbye(&name);
}
