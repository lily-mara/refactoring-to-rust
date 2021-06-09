use http::{Request, Response, StatusCode};
use nginx::{handlers, EasyRequestBody};

handlers! {
  ngx_http_unique_handler;
  "/unique" => is_request_unique,
}

static mut ALREADY_SEEN_BODIES: Vec<EasyRequestBody> = Vec::new(); // <1>

fn is_request_unique(request: Request<EasyRequestBody>) -> Response<String> {
  let request_body = request.into_body();
  let body_str = request_body.as_str().unwrap();

  let mut already_seen = false;

  for body in unsafe { &ALREADY_SEEN_BODIES } {
    let other_body_str = body.as_str().unwrap(); // <2>
    if body_str == other_body_str {
      already_seen = true;
      break;
    }
  }

  unsafe {
    ALREADY_SEEN_BODIES.push(request_body); // <3>
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
