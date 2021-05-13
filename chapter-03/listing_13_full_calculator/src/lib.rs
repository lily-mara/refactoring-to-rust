use libc::{c_char, c_int};
use std::collections::VecDeque;
use std::ffi::CStr;
use std::fmt::{Display, Formatter};

#[no_mangle]
pub extern "C" fn solve(line: *const c_char, solution: *mut c_int) -> c_int {
  if line.is_null() || solution.is_null() {
    return 1;
  }

  let c_str = unsafe { CStr::from_ptr(line) };
  let r_str = match c_str.to_str() {
    Ok(s) => s,
    Err(e) => {
      eprintln!("UTF-8 Error: {}", e);
      return 1;
    }
  };

  match evaluate(r_str) {
    Ok(value) => {
      unsafe {
        *solution = value as c_int;
      }
      0
    }
    Err(e) => {
      eprintln!("Error: {}", e);
      1
    }
  }
}

enum Error {
  InvalidNumber,
  PopFromEmptyStack,
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
    match self {
      Error::InvalidNumber => write!(f, "Not a valid number or operator"),
      Error::PopFromEmptyStack => write!(f, "Tried to operate on empty stack"),
    }
  }
}

#[derive(Debug)]
struct RpnStack {
  stack: VecDeque<i32>,
}

impl RpnStack {
  fn new() -> RpnStack {
    RpnStack {
      stack: VecDeque::new(),
    }
  }

  fn push(&mut self, value: i32) {
    self.stack.push_front(value);
  }

  fn pop(&mut self) -> Result<i32, Error> {
    match self.stack.pop_front() {
      Some(value) => Ok(value),
      None => Err(Error::PopFromEmptyStack),
    }
  }
}

fn evaluate(problem: &str) -> Result<i32, Error> {
  let mut stack = RpnStack::new();

  for term in problem.trim().split(' ') {
    match term {
      "+" => {
        let y = stack.pop()?;
        let x = stack.pop()?;
        stack.push(x + y);
      }
      "-" => {
        let y = stack.pop()?;
        let x = stack.pop()?;
        stack.push(x - y);
      }
      "*" => {
        let y = stack.pop()?;
        let x = stack.pop()?;
        stack.push(x * y);
      }
      "/" => {
        let y = stack.pop()?;
        let x = stack.pop()?;
        stack.push(x / y);
      }
      other => match other.parse() {
        Ok(value) => stack.push(value),
        Err(_) => return Err(Error::InvalidNumber),
      },
    }
  }

  let value = stack.pop()?;
  Ok(value)
}
