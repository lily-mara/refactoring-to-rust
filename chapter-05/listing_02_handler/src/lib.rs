use nginx_sys::{
  ngx_buf_t, ngx_chain_t, ngx_http_output_filter,
  ngx_http_read_client_request_body, ngx_http_request_t, ngx_http_send_header,
  ngx_int_t, ngx_pcalloc, ngx_uint_t, off_t, size_t,
};

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

  let request = &mut *r;

  let body = match request_body_as_str(request) {
    Ok(body) => body,
    Err(e) => {
      eprintln!("failed to parse body: {}", e);
      return;
    }
  };

  match calculate::evaluate(body) {
    Ok(result) => {
      let response_body = format!("{}", result);

      match write_response(request, &response_body, 200) {
        Ok(()) => {}
        Err(e) => {
          eprintln!("failed to write HTTP response: {}", e);
        }
      }
    }
    Err(e) => eprintln!("{} => error: {}", body, e),
  }
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

unsafe fn write_response(
  request: &mut ngx_http_request_t,
  response_body: &str,
  status_code: ngx_uint_t,
) -> Result<(), &'static str> {
  let headers = &mut request.headers_out;

  headers.status = status_code;

  let response_bytes = response_body.as_bytes();
  headers.content_length_n = response_bytes.len() as off_t;

  let rc = ngx_http_send_header(request);
  if rc != 0 {
    return Err("failed to send headers");
  }

  let buf_p =
    ngx_pcalloc(request.pool, std::mem::size_of::<ngx_buf_t>() as size_t)
      as *mut ngx_buf_t;
  if buf_p.is_null() {
    return Err("Failed to allocate buffer");
  }

  let buf = &mut (*buf_p);

  buf.set_last_buf(1);
  buf.set_last_in_chain(1);
  buf.set_memory(1);

  let response_buffer =
    ngx_pcalloc(request.pool, response_bytes.len() as size_t);
  if response_buffer.is_null() {
    return Err("Failed to allocate response buffer");
  }

  std::ptr::copy_nonoverlapping(
    response_bytes.as_ptr(),
    response_buffer as *mut u8,
    response_bytes.len(),
  );

  buf.pos = response_buffer as *mut u8;
  buf.last = response_buffer.offset(response_bytes.len() as isize) as *mut u8;

  let mut out_chain = ngx_chain_t {
    buf,
    next: std::ptr::null_mut(),
  };

  if ngx_http_output_filter(request, &mut out_chain) != 0 {
    return Err("Failed to perform http output filter chain");
  }

  Ok(())
}
