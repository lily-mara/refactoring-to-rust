use http::{Request, Response, StatusCode};
use nginx::{handlers, RequestBody};

handlers! {
  "/calculate" => calculator,
}

fn calculator<'a>(request: Request<RequestBody<'a>>) -> Response<String> {
  match calculate::evaluate(request.body().as_str()) {
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
