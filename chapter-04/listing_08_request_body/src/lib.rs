#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
include!(concat!(env!("OUT_DIR"), "/nginx.rs"));

#[no_mangle]
pub unsafe extern "C" fn ngx_http_calculator_handler(
  r: *mut ngx_http_request_t,
) -> ngx_int_t {
  let rc = ngx_http_read_client_request_body(r, Some(read_body_handler));
  if rc != 0 {
    return rc;
  }

  0
}

unsafe extern "C" fn read_body_handler(r: *mut ngx_http_request_t) {
  if r.is_null() {
    eprintln!("got null request in body handler");
    return;
  }

  let request = &*r;

  let body = match request_body_as_str(request) {
    Ok(body) => body,
    Err(e) => {
      eprintln!("failed to parse body: {}", e);
      return;
    }
  };

  eprintln!("Read request body: {:?}", body);
}

unsafe fn request_body_as_str<'a>(
  request: &'a ngx_http_request_t,
) -> Result<&'a str, &'static str> {
  if request.request_body.is_null()
    || (*request.request_body).bufs.is_null()
    || (*(*request.request_body).bufs).buf.is_null()
  {
    return Err("Request body buffers were not initialized as expected");
  }

  let buf = (*(*request.request_body).bufs).buf;

  let start = (*buf).pos;
  let len = (*buf).last.offset_from(start) as usize;

  let body_bytes = std::slice::from_raw_parts(start, len);

  let body_str = std::str::from_utf8(body_bytes)
    .map_err(|_| "Body contains invalid UTF-8")?;

  Ok(body_str)
}
