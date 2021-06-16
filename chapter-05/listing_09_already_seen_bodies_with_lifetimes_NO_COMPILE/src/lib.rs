use http::{Request, Response, StatusCode};
use nginx::{handler, RequestBody};

handler!(ngx_http_unique_handler, is_request_unique);

static mut ALREADY_SEEN_BODIES: Vec<RequestBody<'static>> = Vec::new();

fn is_request_unique<'a>(
  request: Request<RequestBody<'a>>,
) -> Response<String> {
  let request_body = request.into_body();
  let body_str = request_body.as_str().unwrap();

  let mut already_seen = false;

  for body in unsafe { &ALREADY_SEEN_BODIES } {
    let other_body_str = body.as_str().unwrap();
    if body_str == other_body_str {
      already_seen = true;
      break;
    }
  }

  unsafe {
    ALREADY_SEEN_BODIES.push(request_body);
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
