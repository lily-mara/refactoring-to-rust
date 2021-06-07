use http::{Request, Response, StatusCode};
use nginx::{handlers, RequestBody};

handlers! {
  ngx_http_calculator_handler;
  "/calculate" => calculator,
}

fn calculator<'a>(request: Request<RequestBody<'a>>) -> Response<String> {
  let request_body = match request.body().as_str() {
    Ok(body) => body,
    Err(err) => {
      return Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body(format!("{}", err))
        .unwrap()
    }
  };

  match calculate::evaluate(request_body) {
    Ok(result) => Response::builder()
      .status(StatusCode::OK)
      .body(format!("{}", result))
      .unwrap(),
    Err(e) => Response::builder()
      .status(StatusCode::BAD_REQUEST)
      .body(format!("{}", e))
      .unwrap(),
  }
}
