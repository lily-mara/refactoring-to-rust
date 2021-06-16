use http::{Request, Response, StatusCode};
use nginx::{handler, RequestBody};

handler!(ngx_http_unique_handler, is_request_unique);

static mut ALREADY_SEEN_BODIES: Vec<String> = Vec::new();

fn is_request_unique<'a>(
  request: Request<RequestBody<'a>>,
) -> Response<String> {
  let request_body = request.into_body();
  let body_str = request_body.as_str().unwrap();

  let mut already_seen = false;

  for body in unsafe { &ALREADY_SEEN_BODIES } {
    if body_str == body {
      already_seen = true;
      break;
    }
  }

  unsafe {
    ALREADY_SEEN_BODIES.push(body_str.to_string());
  }

  Response::builder()
    .status(StatusCode::OK)
    .body(String::from(if already_seen {
      "We have already seen that request body"
    } else {
      "This is the first time we've seen that body!"
    }))
    .unwrap()
}
