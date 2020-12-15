fn main() {
  let data: Vec<u8> = vec![5, 10, 15, 20];

  read_u8_slice(data.as_ptr(), data.len());
}

fn read_u8_slice(slice_p: *const u8, length: usize) {
  for index in 0..length {
    unsafe {
      println!("slice[{}] = {}", index, *slice_p.offset(index as isize));
    }
  }
}
