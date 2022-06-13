use pyo3::{prelude::*, types::PyFloat};

#[pyfunction]
fn sleep_for<'p>(py: Python<'p>, id: &'p PyAny) -> PyResult<&'p PyAny> {
  let id: i32 = id.extract()?;
  pyo3_asyncio::tokio::future_into_py(py, async move {
    println!("Enqueue task {id}");

    let total = tokio::task::spawn_blocking(move || {
      println!("Starting task {id}");

      let mut total: u64 = 1;

      for i in 1..1_000_000 {
        for j in 1..100 {
          total += i % j / total;
        }
      }

      total
    })
    .await
    .unwrap();

    Python::with_gil(|py| Ok(PyFloat::new(py, total as f64).into()))
  })
}

#[pymodule]
fn rust_async(_py: Python, m: &PyModule) -> PyResult<()> {
  m.add_function(wrap_pyfunction!(sleep_for, m)?)?;

  Ok(())
}
