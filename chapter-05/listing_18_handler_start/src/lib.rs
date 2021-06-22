use nginx_sys::{
  ngx_buf_t, ngx_chain_t, ngx_http_output_filter, ngx_http_request_t,
  ngx_http_send_header, ngx_pcalloc, ngx_str_t, ngx_uint_t, off_t, size_t,
};
use std::{marker::PhantomData, ptr::NonNull};

#[macro_export]
macro_rules! handler {
  ($c_handler_fn:ident, $rust_handler_fn:ident) => {};
}

pub struct RequestBody<'a> {
  lifetime: PhantomData<&'a ()>,
  request: NonNull<ngx_http_request_t>,
}

impl<'a> std::fmt::Debug for RequestBody<'a> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self.as_str() {
      Ok(s) => write!(f, "{:?}", s),
      Err(e) => write!(f, "<invalid request body: {}>", e),
    }
  }
}

impl<'a> RequestBody<'a> {
  pub fn new(request: NonNull<ngx_http_request_t>) -> Self {
    Self {
      request,
      lifetime: PhantomData,
    }
  }

  pub fn as_str(&self) -> Result<&'a str, &'static str> {
    unsafe {
      let request = self.request.as_ref();

      if request.request_body.is_null()
        || (*request.request_body).bufs.is_null()
        || (*(*request.request_body).bufs).buf.is_null()
      {
        return Err(
          "Request body buffers were not initialized as expected",
        );
      }

      let buf = (*(*request.request_body).bufs).buf;

      let start = (*buf).pos;
      let len = (*buf).last.offset_from(start) as usize;

      let body_bytes = std::slice::from_raw_parts(start, len);

      let body_str = std::str::from_utf8(body_bytes)
        .map_err(|_| "Body contains invalid UTF-8")?;

      Ok(body_str)
    }
  }
}
