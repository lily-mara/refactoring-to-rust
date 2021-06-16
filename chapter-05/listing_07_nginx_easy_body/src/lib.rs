use std::ptr::NonNull;

use nginx_sys::{
  ngx_buf_t, ngx_chain_t, ngx_http_output_filter, ngx_http_request_t,
  ngx_http_send_header, ngx_pcalloc, ngx_str_t, ngx_uint_t, off_t, size_t,
};

#[macro_export]
macro_rules! handler {
  ($c_fn_name:ident, $rs_fn_name:ident) => {
    #[no_mangle]
    pub unsafe extern "C" fn $c_fn_name(
      r: *mut nginx_sys::ngx_http_request_t,
    ) -> nginx_sys::ngx_int_t {
      let rc = nginx_sys::ngx_http_read_client_request_body(
        r,
        Some(read_body_handler),
      );
      if rc != 0 {
        return rc;
      }

      0
    }

    unsafe extern "C" fn read_body_handler(
      r: *mut nginx_sys::ngx_http_request_t,
    ) {
      let request = match std::ptr::NonNull::new(r) {
        Some(request) => request,
        None => {
          eprintln!("got null request in body handler");
          return;
        }
      };

      let request_body = $crate::EasyRequestBody::new(request);

      let uri = match $crate::ngx_str_to_str(&request.as_ref().uri) {
        Some(uri) => uri,
        None => {
          eprintln!("Found invalid URI");
          return;
        }
      };

      use http::method::Method;

      let method = match request.as_ref().method as u32 {
        nginx_sys::NGX_HTTP_GET => Method::GET,
        nginx_sys::NGX_HTTP_POST => Method::POST,
        nginx_sys::NGX_HTTP_PUT => Method::PUT,
        nginx_sys::NGX_HTTP_DELETE => Method::DELETE,
        nginx_sys::NGX_HTTP_HEAD => Method::HEAD,
        nginx_sys::NGX_HTTP_OPTIONS => Method::OPTIONS,
        nginx_sys::NGX_HTTP_PATCH => Method::PATCH,
        nginx_sys::NGX_HTTP_TRACE => Method::TRACE,
        x => {
          eprintln!("Got unknown method type: {}", x);

          Method::GET
        }
      };

      let request = http::Request::builder()
        .uri(uri)
        .method(method)
        .body(request_body)
        .unwrap();

      let response = $rs_fn_name(request);

      if let Err(e) = $crate::write_response(r, response) {
        eprintln!("Failed to write NGINX response object: {}", e);
      }
    }
  };
}

pub struct EasyRequestBody {
  request: NonNull<ngx_http_request_t>,
}

impl std::fmt::Debug for EasyRequestBody {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if let Ok(s) = self.as_str() {
      return write!(f, "{:?}", s);
    }

    if let Ok(b) = self.as_bytes() {
      return write!(f, "{:?}", b);
    }

    write!(f, "<invalid request body>")
  }
}

pub fn ngx_str_to_str(s: &ngx_str_t) -> Option<&str> {
  if s.data.is_null() {
    return None;
  }

  unsafe {
    let bytes = std::slice::from_raw_parts(s.data, s.len as usize);

    std::str::from_utf8(bytes).ok()
  }
}

impl EasyRequestBody {
  pub fn new(request: NonNull<ngx_http_request_t>) -> Self {
    Self { request }
  }

  pub fn as_str(&self) -> Result<&str, &'static str> {
    let body_str = std::str::from_utf8(self.as_bytes()?)
      .map_err(|_| "Body contains invalid UTF-8")?;

    Ok(body_str)
  }

  pub fn as_bytes(&self) -> Result<&[u8], &'static str> {
    unsafe {
      if self.request.as_ref().request_body.is_null()
        || (*self.request.as_ref().request_body).bufs.is_null()
        || (*(*self.request.as_ref().request_body).bufs).buf.is_null()
      {
        return Err("Request body buffers were not initialized as expected");
      }

      let buf = (*(*self.request.as_ref().request_body).bufs).buf;

      let start = (*buf).pos;
      let len = (*buf).last.offset_from(start) as usize;

      let body_bytes = std::slice::from_raw_parts(start, len);

      Ok(body_bytes)
    }
  }
}

pub unsafe fn write_response(
  request: *mut ngx_http_request_t,
  response: http::Response<String>,
) -> Result<(), &'static str> {
  let headers = &mut (*request).headers_out;

  headers.status = response.status().as_u16() as ngx_uint_t;

  let response_bytes = response.body().as_bytes();
  headers.content_length_n = response_bytes.len() as off_t;

  let rc = ngx_http_send_header(request);
  if rc != 0 {
    return Err("failed to send headers");
  }

  let buf_p =
    ngx_pcalloc((*request).pool, std::mem::size_of::<ngx_buf_t>() as size_t)
      as *mut ngx_buf_t;
  if buf_p.is_null() {
    return Err("Failed to allocate buffer");
  }

  let buf = &mut (*buf_p);

  buf.set_last_buf(1);
  buf.set_last_in_chain(1);
  buf.set_memory(1);

  let response_buffer =
    ngx_pcalloc((*request).pool, response_bytes.len() as size_t);
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
