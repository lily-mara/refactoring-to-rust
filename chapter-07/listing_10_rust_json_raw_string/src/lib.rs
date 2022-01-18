use pyo3::prelude::*;

#[derive(Debug, serde::Deserialize)]
struct Data {
  name: String,
  value: i32,
}

#[pyfunction]
fn sum(input: &str) -> i32 {
  let parsed: Data = serde_json::from_str(input).unwrap();

  parsed.name.len() as i32 + parsed.value
}

#[pymodule]
fn rust_json(_py: Python, m: &PyModule) -> PyResult<()> {
  m.add_function(wrap_pyfunction!(sum, m)?)?;

  Ok(())
}

#[cfg(test)]
mod tests {
  use crate::sum;

  #[test]
  fn test_stokes_baker() {
    assert_eq!(
      sum(r#"{ "name": "Stokes Baker", "value": 954832 }"#),
      954844
    );
  }
}
