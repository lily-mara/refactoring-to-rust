use http::{Request, Response, StatusCode};
use nginx::handlers;

handlers! {
  "/calculate" => calculator,
}

fn calculator(request: Request<String>) -> Response<String> {
  match calculate::evaluate(request.body()) {
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
