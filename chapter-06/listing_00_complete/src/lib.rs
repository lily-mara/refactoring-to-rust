use pyo3::{exceptions::PyValueError, prelude::*};
use serde::Deserialize;

#[derive(Deserialize)]
struct Data<'a> {
  name: &'a str,
  value: u16,
}

#[pyfunction]
fn sum_json(input: &str) -> PyResult<u64> {
  let d: Data = serde_json::from_str(input)
    .map_err(|e| PyValueError::new_err(format!("{}", e)))?;

  Ok(d.value as u64 + d.name.len() as u64)
}

#[pymodule]
fn json_summer(_py: Python, m: &PyModule) -> PyResult<()> {
  m.add_function(wrap_pyfunction!(sum_json, m)?)?;

  Ok(())
}
