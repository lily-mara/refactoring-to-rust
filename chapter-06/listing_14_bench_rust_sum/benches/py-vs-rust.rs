use criterion::{
  black_box, criterion_group, criterion_main, Bencher, Criterion,
};
use pyo3::prelude::*;
use pyo3::types::PyDict;

criterion_main!(python_vs_rust);
criterion_group!(python_vs_rust, bench_fn);

fn bench_py(b: &mut Bencher, code: &str, input: &str) {
  Python::with_gil(|py| {
    let locals = PyDict::new(py);

    locals.set_item("json", py.import("json").unwrap()).unwrap();
    locals
      .set_item("rust_json", py.import("rust_json").unwrap())
      .unwrap();
    locals.set_item("INPUT", input).unwrap();

    b.iter(|| black_box(py.run(code, None, Some(&locals)).unwrap()));
  });
}

fn bench_fn(c: &mut Criterion) {
  let input = r#"{"name": "lily", "value": 42}"#;

  c.bench_function("pure python", |b| {
    bench_py(
      b,
      "
value = json.loads(INPUT)
s = value['value'] + len(value['name'])
      ",
      input,
    );
  });

  c.bench_function("rust extension library", |b| {
    bench_py(b, "s = rust_json.sum(INPUT)", input);
  });
}
