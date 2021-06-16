struct PointerWrapper {
  ptr: *const i32,
}

impl PointerWrapper {
  pub fn new(ptr: *const i32) -> Self {
    Self { ptr }
  }

  pub fn get(&self) -> i32 {
    unsafe { *self.ptr }
  }
}

fn main() {
  let p = PointerWrapper::new(std::ptr::null());

  println!("{}", p.get());
}
