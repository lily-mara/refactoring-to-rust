use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

fn main() {
  let mut f = File::open("data.csv").unwrap();
  let mut s = String::new();

  f.read_to_string(&mut s).unwrap();

  let start = Instant::now();
  let sum = sum_csv_column(&s, 10);
  let end = Instant::now();

  println!("{:?}", end - start);
  println!("{:?}", sum);
}

fn sum_csv_column(data: &str, column: usize) -> i64 {
  let mut sum = 0;

  for line in data.lines() {
    if line.len() == 0 {
      continue;
    }

    let value_str = line.split(",").nth(column).unwrap();
    sum += value_str.parse::<i64>().unwrap();
  }

  sum
}
