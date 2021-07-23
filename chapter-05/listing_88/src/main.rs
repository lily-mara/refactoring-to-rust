use input::{get_name, how_was_day, DayKind};
use output::{bad_day, good_day, goodbye, hello};

mod input;
mod output;

fn main() {
  let name = get_name();

  hello(&name);

  match how_was_day() {
    DayKind::Good => good_day(),
    DayKind::Bad => bad_day(),
  }

  goodbye(&name);
}
