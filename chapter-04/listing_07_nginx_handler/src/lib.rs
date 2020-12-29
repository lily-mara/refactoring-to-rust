#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
include!(concat!(env!("OUT_DIR"), "/nginx.rs"));

#[no_mangle]
pub unsafe extern "C" fn ngx_http_calculator_handler(
  r: *mut ngx_http_request_t,
) -> ngx_int_t {
  eprintln!("Hello from Rust!");

  0
}
